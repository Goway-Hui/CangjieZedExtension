use serde_json::Value;
use zed_extension_api::Worktree;

use crate::{
    config::{get_cangjie_home, get_lsp_server_path},
    util::{is_windows, path_sep, server_name},
};

/// Build the path to the LSPServer binary, resolving in order:
/// 1. User-configured `lsp_server_path`
/// 2. `CANGJIE_HOME`/tools/bin/LSPServer
/// 3. PATH lookup for LSPServer
pub fn find_lsp_server(
    configuration: &Option<Value>,
    worktree: &Worktree,
) -> Result<String, String> {
    // 1. User-configured explicit path
    if let Some(path) = get_lsp_server_path(configuration, worktree) {
        return Ok(path);
    }

    let env = worktree.shell_env();
    let is_win = is_windows(&env);
    let name = server_name(is_win);

    // 2. CANGJIE_HOME
    if let Some(home) = get_cangjie_home(configuration, worktree) {
        let sep = path_sep(is_win);
        let path = format!("{home}{sep}tools{sep}bin{sep}{name}");
        return Ok(path);
    }

    // 3. PATH lookup
    if let Some(path) = worktree.which(name) {
        return Ok(path);
    }

    Err(format!(
        "LSPServer not found. Set CANGJIE_HOME or configure lsp_server_path in LSP settings.\n\n\
         Install the Cangjie SDK from https://cangjie-lang.cn/"
    ))
}

/// Build the SDK library paths to prepend to PATH so LSPServer can find its DLLs.
pub fn sdk_library_paths(cangjie_home: &str, is_win: bool) -> Vec<String> {
    let sep = path_sep(is_win);
    let platform_lib = if is_win {
        "windows_x86_64_cjnative"
    } else {
        "linux_x86_64_cjnative"
    };

    vec![
        format!("{cangjie_home}{sep}runtime{sep}lib{sep}{platform_lib}"),
        format!("{cangjie_home}{sep}lib{sep}{platform_lib}"),
        format!("{cangjie_home}{sep}bin"),
        format!("{cangjie_home}{sep}tools{sep}bin"),
        format!("{cangjie_home}{sep}tools{sep}lib"),
    ]
}

/// Prepend SDK library paths to PATH in environment.
pub fn with_sdk_paths(
    env: Vec<(String, String)>,
    cangjie_home: &str,
    is_win: bool,
) -> Vec<(String, String)> {
    let paths = sdk_library_paths(cangjie_home, is_win);
    let path_sep_str = if is_win { ";" } else { ":" };
    let sdk_prefix = paths.join(path_sep_str);

    let mut result = Vec::with_capacity(env.len());
    for (k, v) in env {
        if k.eq_ignore_ascii_case("PATH") || k.eq_ignore_ascii_case("Path") {
            result.push((k, format!("{sdk_prefix}{path_sep_str}{v}")));
        } else {
            result.push((k, v));
        }
    }
    result
}

/// Build LSP launch arguments for LSPServer.
pub fn lsp_args() -> Vec<String> {
    vec![
        "--enable-log=true".to_string(),
        "--log-path=D:/cangjie-lspserver.log".to_string(),
    ]
}

/// Build multiModuleOption from cjpm.toml for cross-module support.
pub fn build_multi_module(
    worktree: &Worktree,
    root_path: &str,
    sep: char,
    workspace_uri: &str,
) -> serde_json::Map<String, serde_json::Value> {
    let mut modules = serde_json::Map::new();

    let cjpm_content = worktree
        .read_text_file("cjpm.toml")
        .ok()
        .unwrap_or_default();

    let package_name = parse_package_name(&cjpm_content).unwrap_or_else(|| {
        root_path
            .rsplit(sep)
            .next()
            .unwrap_or("workspace")
            .to_string()
    });

    let deps = parse_path_dependencies(&cjpm_content);

    let mut requires = serde_json::Map::new();
    for (dep_name, dep_rel_path) in &deps {
        if let Some((dep_abs, dep_pkg_name)) =
            resolve_module(worktree, root_path, sep, dep_name, dep_rel_path)
        {
            let dep_uri = format!("file:///{}", dep_abs.replace('\\', "/"));
            requires.insert(dep_name.clone(), serde_json::json!({ "path": dep_uri }));
            modules
                .entry(&dep_uri)
                .or_insert_with(|| serde_json::json!({ "name": dep_pkg_name, "requires": {} }));
        }
    }

    modules.entry(workspace_uri.to_string()).or_insert_with(|| {
        serde_json::json!({ "name": package_name, "requires": requires })
    });

    modules
}

fn parse_package_name(content: &str) -> Option<String> {
    let mut in_package = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "[package]" {
            in_package = true;
            continue;
        }
        if in_package {
            if trimmed.starts_with('[') {
                break;
            }
            if let Some(name) = trimmed.strip_prefix("name") {
                if let Some(value) = name.split('=').nth(1) {
                    let v = value.trim().trim_matches('"');
                    if !v.is_empty() {
                        return Some(v.to_string());
                    }
                }
            }
        }
    }
    None
}

fn parse_path_dependencies(content: &str) -> Vec<(String, String)> {
    let mut deps = Vec::new();
    let mut current_dep: Option<&str> = None;

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            let section = &trimmed[1..trimmed.len() - 1];
            for prefix in &["dependencies.", "dev-dependencies."] {
                if let Some(name) = section.strip_prefix(prefix) {
                    current_dep = Some(name);
                }
            }
            if !section.starts_with("dependencies.") && !section.starts_with("dev-dependencies.") {
                current_dep = None;
            }
            continue;
        }
        if let Some(dep_name) = current_dep {
            if let Some(rest) = trimmed.strip_prefix("path") {
                if let Some(value) = rest.split('=').nth(1) {
                    let path = value.trim().trim_matches('"');
                    if !path.is_empty() {
                        deps.push((dep_name.to_string(), path.to_string()));
                    }
                }
            }
        }
    }
    deps
}

fn resolve_module(
    worktree: &Worktree,
    root_path: &str,
    sep: char,
    dep_name: &str,
    dep_rel_path: &str,
) -> Option<(String, String)> {
    let dep_abs = if dep_rel_path.starts_with('.') || dep_rel_path.contains("..") {
        format!("{root_path}{sep}{dep_rel_path}").replace('/', &sep.to_string())
    } else {
        format!("{root_path}{sep}{dep_rel_path}")
    };

    let normalized = dep_rel_path.replace('\\', "/");
    let dep_toml_path = format!("{normalized}/cjpm.toml");

    let dep_content = worktree.read_text_file(&dep_toml_path).ok()?;
    let dep_pkg_name =
        parse_package_name(&dep_content).unwrap_or_else(|| dep_name.to_string());

    Some((dep_abs, dep_pkg_name))
}
