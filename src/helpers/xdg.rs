use std::ffi::OsStr;
use std::path::PathBuf;

fn path_from_var<S: AsRef<OsStr>>(key: S, default: S) -> Option<PathBuf> {
    if let Some(path_dir) = std::env::var_os(key) {
        Some(path_dir.into())
    } else if let Some(mut home_dir) = std::env::home_dir() {
        home_dir.push(default.as_ref());
        Some(home_dir)
    } else {
        None
    }
}

pub(super) fn config_home() -> Option<PathBuf> {
    path_from_var("XDG_CONFIG_HOME", ".config")
}

pub(super) fn cache_home() -> Option<PathBuf> {
    path_from_var("XDG_CACHE_HOME", ".cache")
}
