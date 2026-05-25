use percent_encoding::utf8_percent_encode;
use std::{
    env::current_dir,
    fs,
    path::{Path, PathBuf},
};
use zed_extension_api::{
    self as zed, Os, Worktree, current_platform,
};

const EXPAND_ERROR: &str = "Failed to expand ~";
const CURR_DIR_ERROR: &str = "Could not get current dir";
const DIR_ENTRY_LOAD_ERROR: &str = "Failed to load directory entry";
const DIR_ENTRY_RM_ERROR: &str = "Failed to remove directory entry";
const ENTRY_TYPE_ERROR: &str = "Could not determine entry type";
const FILE_ENTRY_RM_ERROR: &str = "Failed to remove file entry";
const PATH_TO_STR_ERROR: &str = "Failed to convert path to string";
const PATH_IS_NOT_DIR: &str = "File exists but is not a path";

const ONCE_CHECK_MARKER: &str = ".update_checked";

/// Detect host OS at runtime from environment variables.
/// `cfg!(windows)` is always false on wasm32-wasip2 target.
pub fn is_windows(env: &[(String, String)]) -> bool {
    env.iter()
        .any(|(k, v)| k == "OS" && v.contains("Windows"))
}

pub fn create_path_if_not_exists<P: AsRef<Path>>(path: P) -> zed::Result<()> {
    let path_ref = path.as_ref();
    match fs::metadata(path_ref) {
        Ok(metadata) => {
            if metadata.is_dir() {
                Ok(())
            } else {
                Err(format!("{PATH_IS_NOT_DIR}: {path_ref:?}"))
            }
        }
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            fs::create_dir_all(path_ref).map_err(|e| e.to_string())
        }
        Err(e) => Err(e.to_string()),
    }
}

pub fn has_checked_once(component_name: &str) -> bool {
    PathBuf::from(component_name).join(ONCE_CHECK_MARKER).exists()
}

pub fn mark_checked_once(component_name: &str, version: &str) -> zed::Result<()> {
    let marker_path = PathBuf::from(component_name).join(ONCE_CHECK_MARKER);
    create_path_if_not_exists(PathBuf::from(component_name))
        .map_err(|err| format!("Failed to create directory for {component_name}: {err}"))?;
    fs::write(&marker_path, version)
        .map_err(|err| format!("Failed to write marker file {marker_path:?}: {err}"))
}

pub fn expand_home_path(worktree: &Worktree, path: String) -> zed::Result<String> {
    match current_platform() {
        (Os::Windows, _) => Ok(path),
        (_, _) => worktree
            .shell_env()
            .iter()
            .find(|&(key, _)| key == "HOME")
            .map_or_else(
                || Err(EXPAND_ERROR.to_string()),
                |(_, value)| Ok(path.replace("~", value)),
            ),
    }
}

pub fn get_curr_dir() -> zed::Result<PathBuf> {
    current_dir().map_err(|_| CURR_DIR_ERROR.to_string())
}

pub fn path_to_string<P: AsRef<Path>>(path: P) -> zed::Result<String> {
    path.as_ref()
        .to_path_buf()
        .into_os_string()
        .into_string()
        .map_err(|_| PATH_TO_STR_ERROR.to_string())
}

const PATH_ENCODE_SET: percent_encoding::AsciiSet = percent_encoding::NON_ALPHANUMERIC
    .remove(b'/')
    .remove(b':')
    .remove(b'-')
    .remove(b'.')
    .remove(b'_')
    .remove(b'~')
    .remove(b'@');

pub fn path_to_file_uri(path: &str) -> String {
    let mut uri = String::with_capacity(path.len() + 8);
    uri.push_str("file://");
    if path.starts_with('/') {
        uri.extend(utf8_percent_encode(path, &PATH_ENCODE_SET));
    } else {
        for chunk in path.split('\\') {
            uri.push('/');
            uri.extend(utf8_percent_encode(chunk, &PATH_ENCODE_SET));
        }
    }
    uri
}

pub fn remove_all_files_except<P: AsRef<Path>>(prefix: P, filename: &str) -> zed::Result<()> {
    let entries: Vec<_> = match fs::read_dir(prefix) {
        Ok(entries) => entries.filter_map(|e| e.ok()).collect(),
        Err(err) => {
            println!("{DIR_ENTRY_LOAD_ERROR}: {err}");
            return Err(format!("{DIR_ENTRY_LOAD_ERROR}: {err}"));
        }
    };

    for entry in entries {
        if entry.file_name().to_str() != Some(filename) {
            if let Ok(t) = entry.file_type() {
                if t.is_dir() {
                    if let Err(err) = fs::remove_dir_all(entry.path()) {
                        println!("{DIR_ENTRY_RM_ERROR}: {err}");
                    }
                } else if t.is_file() {
                    if let Err(err) = fs::remove_file(entry.path()) {
                        println!("{FILE_ENTRY_RM_ERROR}: {err}");
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn server_name(is_windows: bool) -> &'static str {
    if is_windows {
        "LSPServer.exe"
    } else {
        "LSPServer"
    }
}

pub fn path_sep(is_windows: bool) -> char {
    if is_windows { '\\' } else { '/' }
}

pub fn parent_dir(path: &str, is_windows: bool) -> &str {
    if is_windows {
        // Paths may use / or \ on Windows — try both.
        path.rfind('\\')
            .or_else(|| path.rfind('/'))
            .map(|i| &path[..i])
            .unwrap_or(".")
    } else {
        path.rfind('/').map(|i| &path[..i]).unwrap_or(".")
    }
}
