use flate2::read::GzDecoder;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use tar::Archive;

/// Extract a `.tar.gz` archive into `dest_dir` and return the path to the
/// first executable file found whose name starts with "unilang".
pub fn extract_targz(archive_path: &str, dest_dir: &str) -> Result<String, String> {
    let file = fs::File::open(archive_path).map_err(|e| format!("Cannot open archive: {}", e))?;
    let gz = GzDecoder::new(file);
    let mut archive = Archive::new(gz);

    archive
        .unpack(dest_dir)
        .map_err(|e| format!("Failed to extract tar.gz: {}", e))?;

    find_unilang_binary(dest_dir)
}

/// Extract a `.zip` archive into `dest_dir` and return the path to the
/// first file whose name starts with "unilang".
#[cfg(target_os = "windows")]
pub fn extract_zip(archive_path: &str, dest_dir: &str) -> Result<String, String> {
    use zip::ZipArchive;

    let file =
        fs::File::open(archive_path).map_err(|e| format!("Cannot open zip archive: {}", e))?;
    let mut zip = ZipArchive::new(file).map_err(|e| format!("Invalid zip archive: {}", e))?;

    for i in 0..zip.len() {
        let mut entry = zip
            .by_index(i)
            .map_err(|e| format!("Zip read error: {}", e))?;
        let out_path = Path::new(dest_dir).join(entry.name());
        if entry.is_dir() {
            fs::create_dir_all(&out_path).map_err(|e| format!("Cannot create directory: {}", e))?;
        } else {
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Cannot create directory: {}", e))?;
            }
            let mut outfile =
                fs::File::create(&out_path).map_err(|e| format!("Cannot write file: {}", e))?;
            io::copy(&mut entry, &mut outfile)
                .map_err(|e| format!("Cannot extract file: {}", e))?;
        }
    }

    find_unilang_binary(dest_dir)
}

/// On non-Windows we provide a stub so the symbol always exists, but we only
/// call it on Windows.
#[cfg(not(target_os = "windows"))]
pub fn extract_zip(_archive_path: &str, _dest_dir: &str) -> Result<String, String> {
    Err("ZIP extraction is only supported on Windows".to_string())
}

/// Walk `dir` recursively to find the first file that looks like the unilang
/// binary (name starts with "unilang" and is a file, not a directory).
fn find_unilang_binary(dir: &str) -> Result<String, String> {
    let base = Path::new(dir);
    find_binary_recursive(base)
        .map_err(|e| format!("Error scanning extracted archive: {}", e))?
        .map(|p| p.to_string_lossy().to_string())
        .ok_or_else(|| {
            format!(
                "Could not find a 'unilang' binary inside the extracted archive at '{}'",
                dir
            )
        })
}

fn find_binary_recursive(dir: &Path) -> io::Result<Option<PathBuf>> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let meta = entry.metadata()?;
        if meta.is_dir() {
            if let Some(found) = find_binary_recursive(&path)? {
                return Ok(Some(found));
            }
        } else if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            // Accept "unilang", "unilang-lite", "unilang-full", "unilang.exe" etc.
            if name.starts_with("unilang") {
                return Ok(Some(path));
            }
        }
    }
    Ok(None)
}

/// Copy the extracted binary to `dest_dir`, name it `unilang` (or
/// `unilang.exe` on Windows), set executable permissions on Unix, and return
/// the full destination path.
pub fn install_binary(src: &str, dest_dir: &str) -> Result<String, String> {
    fs::create_dir_all(dest_dir)
        .map_err(|e| format!("Cannot create install directory '{}': {}", dest_dir, e))?;

    let bin_name = if cfg!(target_os = "windows") {
        "unilang.exe"
    } else {
        "unilang"
    };

    let dest_path = Path::new(dest_dir).join(bin_name);
    fs::copy(src, &dest_path).map_err(|e| {
        format!(
            "Cannot copy binary to '{}': {}. You may need to run with sudo.",
            dest_path.display(),
            e
        )
    })?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&dest_path)
            .map_err(|e| format!("Cannot read file metadata: {}", e))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&dest_path, perms)
            .map_err(|e| format!("Cannot set executable permission: {}", e))?;
    }

    Ok(dest_path.to_string_lossy().to_string())
}

/// Return a human-readable hint about updating `PATH` for the given directory.
pub fn path_hint(install_dir: &str) -> String {
    let shell_rc = if cfg!(target_os = "windows") {
        return format!(
            "Add '{}' to your system PATH via System Properties > Environment Variables.",
            install_dir
        );
    } else {
        // Try to detect the user's shell.
        std::env::var("SHELL").unwrap_or_default()
    };

    let rc_file = if shell_rc.contains("zsh") {
        "~/.zshrc"
    } else if shell_rc.contains("fish") {
        "~/.config/fish/config.fish"
    } else {
        "~/.bashrc"
    };

    format!(
        "Add UniLang to your PATH by adding this line to {}:\n\n    export PATH=\"{}:$PATH\"\n\nThen restart your shell or run:  source {}",
        rc_file, install_dir, rc_file
    )
}
