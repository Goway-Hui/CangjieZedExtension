use std::{fs::metadata, path::PathBuf};

use serde_json::Value;
use zed_extension_api::{
    self as zed, DownloadedFileType, GithubReleaseOptions, LanguageServerId,
    LanguageServerInstallationStatus, Worktree,
    set_language_server_installation_status,
};

use crate::{
    config::CheckUpdates,
    util::{mark_checked_once, remove_all_files_except},
};

const PROXY_BINARY: &str = "cangjie-lsp-proxy";
const PROXY_INSTALL_PATH: &str = "proxy-bin";
const GITHUB_REPO: &str = "Goway_Hui/CangjieTreeSitter";

fn asset_name() -> zed::Result<(String, DownloadedFileType)> {
    let (os, arch) = zed::current_platform();
    let (os_str, file_type) = match os {
        zed::Os::Mac => ("darwin", DownloadedFileType::GzipTar),
        zed::Os::Linux => ("linux", DownloadedFileType::GzipTar),
        zed::Os::Windows => ("windows", DownloadedFileType::Zip),
    };
    let arch_str = match arch {
        zed::Architecture::Aarch64 => "aarch64",
        zed::Architecture::X8664 => "x86_64",
        _ => return Err("Unsupported architecture".into()),
    };
    let ext = if matches!(file_type, DownloadedFileType::Zip) {
        "zip"
    } else {
        "tar.gz"
    };
    Ok((
        format!("cangjie-lsp-proxy-{os_str}-{arch_str}.{ext}"),
        file_type,
    ))
}

fn proxy_exec() -> String {
    let (os, _arch) = zed::current_platform();
    match os {
        zed::Os::Linux | zed::Os::Mac => PROXY_BINARY.to_string(),
        zed::Os::Windows => format!("{PROXY_BINARY}.exe"),
    }
}

fn find_latest_local() -> Option<PathBuf> {
    let local_binary = PathBuf::from(PROXY_INSTALL_PATH).join(proxy_exec());
    if metadata(&local_binary).is_ok_and(|m| m.is_file()) {
        return Some(local_binary);
    }

    std::fs::read_dir(PROXY_INSTALL_PATH)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path().join(proxy_exec()))
        .filter(|p| metadata(p).is_ok_and(|m| m.is_file()))
        .last()
}

pub fn binary_path(
    cached: &mut Option<String>,
    configuration: &Option<Value>,
    language_server_id: &LanguageServerId,
    worktree: &Worktree,
) -> zed::Result<String> {
    let exe = proxy_exec();

    // 1. Check alongside extension.wasm (current dir in Zed extension context)
    let cwd_binary = std::env::current_dir()
        .ok()
        .map(|d| d.join(&exe))
        .filter(|p| metadata(p).is_ok_and(|m| m.is_file()));
    if let Some(path) = cwd_binary {
        let s = path.to_string_lossy().to_string();
        *cached = Some(s.clone());
        return Ok(s);
    }

    // 2. Check proxy-bin directory (managed downloads)
    let policy = crate::config::get_check_updates(configuration);
    match policy {
        CheckUpdates::Never => {
            if let Some(path) = find_latest_local() {
                let s = path.to_string_lossy().to_string();
                *cached = Some(s.clone());
                return Ok(s);
            }
        }
        CheckUpdates::Once => {
            if let Some(path) = find_latest_local() {
                let s = path.to_string_lossy().to_string();
                *cached = Some(s.clone());
                return Ok(s);
            }
            if crate::util::has_checked_once(PROXY_INSTALL_PATH) {
                // Already checked — fall through
            }
        }
        CheckUpdates::Always => {}
    }

    // 3. Auto-download from GitHub releases
    if let Ok((name, file_type)) = asset_name() {
        if let Ok(release) = zed::latest_github_release(
            GITHUB_REPO,
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        ) {
            let bin_path = format!("{PROXY_INSTALL_PATH}/{}/{}", release.version, exe);
            if metadata(&bin_path).is_ok() {
                *cached = Some(bin_path.clone());
                return Ok(bin_path);
            }

            if let Some(asset) = release.assets.iter().find(|a| a.name == name) {
                let version_dir = format!("{PROXY_INSTALL_PATH}/{}", release.version);

                set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Downloading,
                );

                if zed::download_file(&asset.download_url, &version_dir, file_type).is_ok() {
                    let _ = zed::make_file_executable(&bin_path);
                    set_language_server_installation_status(
                        language_server_id,
                        &LanguageServerInstallationStatus::None,
                    );
                    let _ = remove_all_files_except(PROXY_INSTALL_PATH, &release.version);
                    let _ = mark_checked_once(PROXY_INSTALL_PATH, &release.version);
                    *cached = Some(bin_path.clone());
                    return Ok(bin_path);
                }
            }
        }
    }

    // 4. Fallback: proxy-bin local install
    if let Some(path) = find_latest_local() {
        let s = path.to_string_lossy().to_string();
        *cached = Some(s.clone());
        return Ok(s);
    }

    // 5. Fallback: binary on $PATH
    if let Some(path) = worktree.which(exe.as_str()) {
        return Ok(path);
    }

    // 6. Stale cache fallback
    if let Some(path) = cached.as_deref() {
        if metadata(path).is_ok() {
            return Ok(path.to_string());
        }
    }

    Err(format!("'{exe}' not found in extension directory, proxy-bin, or PATH"))
}
