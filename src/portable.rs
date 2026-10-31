//! Portable mode: keep every file EzMouse creates next to the program.
//!
//! On Windows, EzMouse never writes to `%LOCALAPPDATA%`, the registry or
//! the Start Menu. Its config, its security certificate (identity) and its
//! crash log all live in a `data` folder beside `ezmouse.exe`, so the whole
//! app can be copied to another PC / USB stick, or deleted by removing the
//! folder.
//!
//! Resolution order:
//!   1. `EZMOUSE_DATA_DIR` environment variable (any OS)
//!   2. Windows: `<folder containing ezmouse.exe>\data`
//!   3. Other OSes: `None` (callers keep their normal XDG / Library paths)
//!
//! If the program folder is read-only (e.g. it was placed in
//! `C:\Program Files`), Windows falls back to `%LOCALAPPDATA%\EzMouse` so the
//! app still works, and a warning is logged.

use std::{
    env, fs,
    path::{Path, PathBuf},
};

/// Name of the data folder created next to the executable.
pub const DATA_DIR_NAME: &str = "data";

/// Returns the directory where EzMouse should keep its files, if portable
/// mode applies on this platform. The directory is created if missing.
pub fn data_dir() -> Option<PathBuf> {
    if let Some(dir) = env::var_os("EZMOUSE_DATA_DIR").filter(|d| !d.is_empty()) {
        let dir = PathBuf::from(dir);
        return ensure_writable(&dir).then_some(dir);
    }
    platform_data_dir()
}

#[cfg(windows)]
fn platform_data_dir() -> Option<PathBuf> {
    let exe = env::current_exe().ok()?;
    let dir = exe.parent()?.join(DATA_DIR_NAME);
    if ensure_writable(&dir) {
        return Some(dir);
    }
    // Program folder is read-only: fall back rather than fail to start.
    let fallback = PathBuf::from(env::var_os("LOCALAPPDATA")?).join("EzMouse");
    log::warn!(
        "{dir:?} is not writable (is EzMouse in a protected folder?); \
         falling back to {fallback:?}. Move the EzMouse folder somewhere \
         you can write to (e.g. Desktop or a USB drive) to stay portable."
    );
    ensure_writable(&fallback).then_some(fallback)
}

#[cfg(not(windows))]
fn platform_data_dir() -> Option<PathBuf> {
    None
}

/// Where the crash log goes in portable mode.
pub fn log_dir() -> Option<PathBuf> {
    data_dir().map(|d| d.join("logs"))
}

fn ensure_writable(dir: &Path) -> bool {
    if fs::create_dir_all(dir).is_err() {
        return false;
    }
    // create_dir_all succeeds on read-only existing dirs, so probe with a file.
    let probe = dir.join(".write-test");
    let ok = fs::write(&probe, b"").is_ok();
    let _ = fs::remove_file(probe);
    ok
}
