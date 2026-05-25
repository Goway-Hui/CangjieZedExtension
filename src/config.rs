use zed_extension_api::{serde_json::Value, Worktree};

use crate::util::expand_home_path;

#[derive(Debug, Clone, PartialEq, Default)]
pub enum CheckUpdates {
    #[default]
    Always,
    Once,
    Never,
}

/// Read `cangjie_home` from LSP settings. Falls back to `CANGJIE_HOME` env var.
pub fn get_cangjie_home(configuration: &Option<Value>, worktree: &Worktree) -> Option<String> {
    // 1. LSP settings
    if let Some(configuration) = configuration {
        if let Some(cangjie_home) = configuration
            .pointer("/cangjie_home")
            .and_then(|x| x.as_str())
        {
            match expand_home_path(worktree, cangjie_home.to_string()) {
                Ok(path) => return Some(path),
                Err(err) => {
                    println!("{err}");
                }
            }
        }
    }

    // 2. worktree shell env (Zed sandbox)
    if let Some(home) = worktree
        .shell_env()
        .into_iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("CANGJIE_HOME"))
        .and_then(|(_, v)| if v.is_empty() { None } else { Some(v) })
    {
        return Some(home);
    }

    // 3. system env (last resort)
    if let Ok(home) = std::env::var("CANGJIE_HOME") {
        if !home.is_empty() {
            return Some(home);
        }
    }

    None
}

/// Read `lsp_server_path` from LSP settings. User-configured path to LSPServer binary.
pub fn get_lsp_server_path(
    configuration: &Option<Value>,
    worktree: &Worktree,
) -> Option<String> {
    if let Some(configuration) = configuration {
        if let Some(path) = configuration
            .pointer("/lsp_server_path")
            .and_then(|x| x.as_str())
        {
            match expand_home_path(worktree, path.to_string()) {
                Ok(p) => return Some(p),
                Err(err) => {
                    println!("{err}");
                }
            }
        }
    }
    None
}

pub fn is_autodownload(configuration: &Option<Value>) -> bool {
    configuration
        .as_ref()
        .and_then(|c| c.pointer("/sdk_auto_download").and_then(|x| x.as_bool()))
        .unwrap_or(false)
}

pub fn get_check_updates(configuration: &Option<Value>) -> CheckUpdates {
    if let Some(configuration) = configuration {
        if let Some(mode_str) = configuration
            .pointer("/check_updates")
            .and_then(|x| x.as_str())
            .map(|s| s.to_lowercase())
        {
            return match mode_str.as_str() {
                "once" => CheckUpdates::Once,
                "never" => CheckUpdates::Never,
                "always" => CheckUpdates::Always,
                _ => CheckUpdates::default(),
            };
        }
    }
    CheckUpdates::default()
}
