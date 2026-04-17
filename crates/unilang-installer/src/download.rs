use indicatif::{ProgressBar, ProgressStyle};
use serde::Deserialize;
use std::fs;
use std::io::{Read, Write};

const GITHUB_REPO: &str = "AIWithHitesh/unilang";

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
}

pub struct ReleaseInfo {
    pub tag: String,
    pub version: String,
}

/// Fetch the latest release tag from the GitHub API.
/// Falls back to the crate's own version if the network call fails.
pub fn fetch_latest_release(repo: &str) -> Result<ReleaseInfo, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);
    match ureq::get(&url)
        .set("User-Agent", "unilang-installer")
        .call()
    {
        Ok(response) => {
            let release: GitHubRelease = response
                .into_json()
                .map_err(|e| format!("Failed to parse GitHub API response: {}", e))?;
            let tag = release.tag_name;
            let version = tag.trim_start_matches('v').to_string();
            Ok(ReleaseInfo { tag, version })
        }
        Err(e) => {
            let fallback_version = env!("CARGO_PKG_VERSION");
            eprintln!(
                "Warning: could not reach GitHub API ({}). Falling back to version v{}.",
                e, fallback_version
            );
            Ok(ReleaseInfo {
                tag: format!("v{}", fallback_version),
                version: fallback_version.to_string(),
            })
        }
    }
}

/// Build the download URL for the binary archive.
pub fn binary_url(tag: &str, edition: &str, target: &str) -> String {
    let ext = if target.ends_with("windows") {
        "zip"
    } else {
        "tar.gz"
    };
    format!(
        "https://github.com/{}/releases/download/{}/unilang-{}-{}.{}",
        GITHUB_REPO, tag, edition, target, ext
    )
}

/// Detect the current OS + architecture as the target string used in release filenames.
pub fn detect_target() -> &'static str {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("linux", "x86_64") => "x86_64-linux",
        ("linux", "aarch64") => "aarch64-linux",
        ("macos", "x86_64") => "x86_64-macos",
        ("macos", "aarch64") => "aarch64-macos",
        ("windows", "x86_64") => "x86_64-windows",
        _ => "x86_64-linux",
    }
}

/// Download a file from `url` to `dest_path`, showing an `indicatif` progress bar.
pub fn download_binary(url: &str, dest_path: &str) -> Result<(), String> {
    let response = ureq::get(url)
        .set("User-Agent", "unilang-installer")
        .call()
        .map_err(|e| format!("Download failed: {}", e))?;

    let content_length: Option<u64> = response
        .header("Content-Length")
        .and_then(|v| v.parse().ok());

    let pb = ProgressBar::new(content_length.unwrap_or(0));
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )
        .unwrap()
        .progress_chars("=>-"),
    );

    let mut reader = response.into_reader();
    let mut dest_file =
        fs::File::create(dest_path).map_err(|e| format!("Cannot create temp file: {}", e))?;

    let mut buf = [0u8; 8192];
    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| format!("Read error during download: {}", e))?;
        if n == 0 {
            break;
        }
        dest_file
            .write_all(&buf[..n])
            .map_err(|e| format!("Write error during download: {}", e))?;
        pb.inc(n as u64);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}

/// Download and verify the SHA-256 checksum for an already-downloaded file.
/// The checksum file is expected at `{url}.sha256`.
pub fn verify_checksum(binary_url: &str, local_path: &str) -> Result<(), String> {
    let sha_url = format!("{}.sha256", binary_url);
    let response = ureq::get(&sha_url)
        .set("User-Agent", "unilang-installer")
        .call()
        .map_err(|e| format!("Could not fetch checksum file: {}", e))?;

    let body = response
        .into_string()
        .map_err(|e| format!("Could not read checksum body: {}", e))?;
    let expected_hex = body.split_whitespace().next().unwrap_or("").to_string();

    if expected_hex.is_empty() {
        return Err("Checksum file was empty or malformed".to_string());
    }

    let data =
        fs::read(local_path).map_err(|e| format!("Could not read downloaded file: {}", e))?;
    let actual_hex = sha256_hex(&data);

    if actual_hex != expected_hex {
        return Err(format!(
            "Checksum mismatch!\n  expected: {}\n  actual:   {}",
            expected_hex, actual_hex
        ));
    }

    Ok(())
}

/// Compute the SHA-256 hex digest of `data` without pulling in an extra dependency.
fn sha256_hex(data: &[u8]) -> String {
    // Pure-Rust SHA-256 using the standard message-schedule algorithm.
    let hash = sha256(data);
    let mut out = String::with_capacity(64);
    for b in &hash {
        use std::fmt::Write as _;
        let _ = write!(out, "{:02x}", b);
    }
    out
}

fn sha256(data: &[u8]) -> [u8; 32] {
    // Initial hash values (first 32 bits of the fractional parts of the square roots of the first 8 primes).
    let mut h: [u32; 8] = [
        0x6a09e667, 0xbb67ae85, 0x3c6ef372, 0xa54ff53a, 0x510e527f, 0x9b05688c, 0x1f83d9ab,
        0x5be0cd19,
    ];

    // Round constants.
    let k: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    // Pre-processing: add padding.
    let bit_len = (data.len() as u64) * 8;
    let mut msg = data.to_vec();
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0x00);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());

    // Process each 512-bit (64-byte) chunk.
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (i, word) in w[..16].iter_mut().enumerate() {
            *word = u32::from_be_bytes([
                chunk[i * 4],
                chunk[i * 4 + 1],
                chunk[i * 4 + 2],
                chunk[i * 4 + 3],
            ]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh] = h;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(k[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);
            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut result = [0u8; 32];
    for (i, word) in h.iter().enumerate() {
        result[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    result
}
