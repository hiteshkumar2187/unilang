// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.

//! External (dynamic) driver loading — North Star feature.
//!
//! ## Vision
//!
//! UniLang's long-term goal is to allow developers to extend the runtime
//! with new drivers simply by dropping a shared library into
//! `~/.unilang/drivers/` — no recompilation required.
//!
//! ```text
//! ~/.unilang/
//! └── drivers/
//!     ├── unilang-driver-postgres.so      ← drop here, picked up on next run
//!     ├── unilang-driver-influxdb.so
//!     └── unilang-driver-custom-crm.dylib
//! ```
//!
//! The driver shared library exports a single C-ABI symbol:
//! `unilang_driver_init(vm: *mut VM) -> i32`
//! which the host runtime calls to register the driver's built-in functions.
//!
//! ## Current status
//!
//! Dynamic loading is **not yet implemented**.  This module provides the
//! scaffolding and documentation so that the ABI and directory conventions
//! are established now, and the implementation can be added in a future
//! release without breaking changes.
//!
//! Track progress at: <https://github.com/AIWithHitesh/unilang/issues>

use std::path::PathBuf;
use unilang_runtime::vm::VM;

// ── ABI contract ─────────────────────────────────────────────────────────────

/// The ABI contract that every external UniLang driver shared library must
/// follow.
///
/// This constant is intended as living documentation: embed it in error
/// messages, print it via `unilang drivers abi`, or write it to disk as a
/// template file.
pub const EXTERNAL_DRIVER_ABI_DOC: &str = r#"
# UniLang External Driver ABI (v1)

External drivers must be compiled as a cdylib and export:

    #[no_mangle]
    pub extern "C" fn unilang_driver_init(vm: *mut std::ffi::c_void) -> i32 {
        // cast vm pointer, register built-ins, return 0 on success
        0
    }

The driver receives a raw pointer to the VM. Cast it using:
    let vm = unsafe { &mut *(vm as *mut unilang_runtime::vm::VM) };

Then call vm.register_builtin("name", func) for each exported function.
Return 0 on success, non-zero on failure.
"#;

// ── Metadata ─────────────────────────────────────────────────────────────────

/// Metadata about an external driver discovered on disk.
///
/// Returned by [`scan_drivers`] and [`load_external_drivers`].  The `loaded`
/// field reflects whether the shared library was successfully opened and its
/// init symbol was called.  In the current release `loaded` is always `false`
/// because dynamic loading is not yet implemented.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExternalDriverInfo {
    /// Absolute path to the shared-library file.
    pub path: PathBuf,
    /// File name component of the path (e.g. `"unilang-driver-postgres.so"`).
    pub file_name: String,
    /// Whether the driver was successfully loaded and initialised.
    ///
    /// Always `false` in this release; will be `true` once dynamic loading is
    /// implemented.
    pub loaded: bool,
    /// Human-readable error encountered while loading, if any.
    pub error: Option<String>,
}

// ── Directory helpers ─────────────────────────────────────────────────────────

/// Returns the path to the user-level external-drivers directory:
/// `~/.unilang/drivers/`
///
/// On Windows the home directory is read from `USERPROFILE`; on all other
/// platforms `HOME` is used.  If neither variable is set the function falls
/// back to the current working directory so that the call never panics.
///
/// The directory is **not** created by this function — callers that need it to
/// exist should call [`std::fs::create_dir_all`] themselves.
pub fn drivers_dir() -> PathBuf {
    // Prefer $HOME on Unix; fall back to $USERPROFILE on Windows.
    let home = std::env::var("HOME")
        .or_else(|_| std::env::var("USERPROFILE"))
        .unwrap_or_else(|_| ".".to_string());

    PathBuf::from(home).join(".unilang").join("drivers")
}

// ── Scanning ──────────────────────────────────────────────────────────────────

/// Scan [`drivers_dir()`] for platform-appropriate shared-library files and
/// return metadata for each one found.
///
/// Recognised extensions:
/// - `.so`    — Linux / Android
/// - `.dylib` — macOS
/// - `.dll`   — Windows
///
/// Files with other extensions are silently ignored.  If the drivers directory
/// does not exist (which is the common case before a user has installed any
/// external drivers) the function returns an empty `Vec` rather than
/// propagating an error.
///
/// **Note:** this function only enumerates files — it does not open or execute
/// any shared library.
#[allow(dead_code)]
pub fn scan_drivers() -> Vec<ExternalDriverInfo> {
    let dir = drivers_dir();

    let read_dir = match std::fs::read_dir(&dir) {
        Ok(rd) => rd,
        // Directory doesn't exist yet — that's fine, just return empty.
        Err(_) => return Vec::new(),
    };

    let mut drivers = Vec::new();

    for entry in read_dir.flatten() {
        let path = entry.path();

        // Only consider regular files with a recognised shared-library extension.
        let is_shared_lib = path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| matches!(ext, "so" | "dylib" | "dll"))
            .unwrap_or(false);

        if !is_shared_lib {
            continue;
        }

        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("<unknown>")
            .to_string();

        drivers.push(ExternalDriverInfo {
            path,
            file_name,
            loaded: false,
            error: None,
        });
    }

    drivers
}

// ── Loading (future entry point) ──────────────────────────────────────────────

/// Attempt to load all external drivers found in [`drivers_dir()`] into `vm`.
///
/// ## Current behaviour (placeholder)
///
/// Dynamic loading is **not yet implemented**.  This function:
/// 1. Calls [`scan_drivers()`] to discover `.so` / `.dylib` / `.dll` files.
/// 2. Marks every discovered driver with `loaded = false` and a
///    "coming soon" error message.
/// 3. Returns the list so the caller can surface diagnostics if desired.
///
/// ## Future behaviour
///
/// Once dynamic loading is implemented this function will:
/// 1. Open each shared library with `libloading` (or equivalent).
/// 2. Resolve the `unilang_driver_init` symbol (see [`EXTERNAL_DRIVER_ABI_DOC`]).
/// 3. Call the symbol, passing a raw pointer to `vm`.
/// 4. Set `loaded = true` on success, or populate `error` on failure.
#[allow(dead_code)]
pub fn load_external_drivers(vm: &mut VM) -> Vec<ExternalDriverInfo> {
    // The `vm` parameter is accepted now so the function signature is stable
    // and callers don't need to change when real loading is added.
    let _ = vm;

    let mut drivers = scan_drivers();

    for driver in &mut drivers {
        driver.loaded = false;
        driver.error = Some(
            "dynamic driver loading is not yet available in this release — coming soon!"
                .to_string(),
        );
    }

    drivers
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn drivers_dir_is_under_home() {
        let dir = drivers_dir();
        // Should end with .unilang/drivers
        let mut components = dir.components().rev();
        assert_eq!(
            components.next().unwrap().as_os_str(),
            "drivers",
            "last component should be 'drivers'"
        );
        assert_eq!(
            components.next().unwrap().as_os_str(),
            ".unilang",
            "second-to-last component should be '.unilang'"
        );
    }

    #[test]
    fn scan_drivers_returns_empty_for_nonexistent_dir() {
        // Override HOME to a temp path that doesn't have a .unilang/drivers dir.
        // We can't easily override env vars in tests without unsafe, so instead
        // we just assert that scan_drivers() doesn't panic when the dir is absent.
        // The function is documented to return an empty Vec in that case.
        let result = std::panic::catch_unwind(scan_drivers);
        assert!(result.is_ok(), "scan_drivers should not panic");
    }

    #[test]
    fn abi_doc_constant_is_non_empty() {
        assert!(
            !EXTERNAL_DRIVER_ABI_DOC.trim().is_empty(),
            "ABI doc constant should not be empty"
        );
        assert!(
            EXTERNAL_DRIVER_ABI_DOC.contains("unilang_driver_init"),
            "ABI doc should mention the init symbol"
        );
    }
}
