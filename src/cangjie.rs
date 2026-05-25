use zed_extension_api::{
    self as zed, CodeLabel, CodeLabelSpan, Extension, LanguageServerId, Worktree,
    lsp::{Completion, CompletionKind, Symbol, SymbolKind},
    serde_json::{Value, json},
    settings::LspSettings,
};

use crate::{
    config::get_cangjie_home,
    lsp_server::{build_multi_module, find_lsp_server, lsp_args, with_sdk_paths},
    proxy,
    util::{is_windows, parent_dir, path_sep, server_name},
};

pub struct CangjieExtension {
    cached_proxy_path: Option<String>,
}

/// Resolve the proxy binary path.
/// Primary: alongside LSPServer (same directory).
/// Fallback: standard binary_path resolution.
fn resolve_proxy_path(
    cached: &mut Option<String>,
    configuration: &Option<Value>,
    language_server_id: &LanguageServerId,
    worktree: &Worktree,
    server_path: &str,
    is_win: bool,
) -> zed::Result<String> {
    let proxy_name = server_name(is_win).replace("LSPServer", "cangjie-lsp-proxy");
    let sep = path_sep(is_win);
    let server_dir = parent_dir(server_path, is_win);
    let sibling_proxy = format!("{server_dir}{sep}{proxy_name}");

    if std::fs::metadata(&sibling_proxy).is_ok_and(|m| m.is_file()) {
        *cached = Some(sibling_proxy.clone());
        return Ok(sibling_proxy);
    }

    proxy::binary_path(cached, configuration, language_server_id, worktree)
}

impl CangjieExtension {
    fn lsp_server_command_impl(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        let env = worktree.shell_env();
        let is_win = is_windows(&env);

        let configuration =
            self.language_server_workspace_configuration(language_server_id, worktree)?;

        let cangjie_home = get_cangjie_home(&configuration, worktree).ok_or_else(|| {
            "CANGJIE_HOME not set. Install the Cangjie SDK from https://cangjie-lang.cn/\n\
             or set cangjie_home in LSP settings."
                .to_string()
        })?;

        // Resolve LSPServer path first — we derive the proxy path from it
        let server_path =
            find_lsp_server(&configuration, worktree)
                .map_err(|err| format!("Failed to find LSPServer: {err}"))?;

        let proxy_path = resolve_proxy_path(
            &mut self.cached_proxy_path,
            &configuration,
            language_server_id,
            worktree,
            &server_path,
            is_win,
        )
        .map_err(|err| format!("Failed to get proxy binary path: {err}"))?;

        let env_vars = with_sdk_paths(env, &cangjie_home, is_win);

        // proxy takes: server_path, [lsp_args...]
        let mut args = vec![server_path];
        args.extend(lsp_args());

        Ok(zed::Command {
            command: proxy_path,
            args,
            env: env_vars,
        })
    }
}

impl Extension for CangjieExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            cached_proxy_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<zed::Command> {
        self.lsp_server_command_impl(language_server_id, worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        let env = worktree.shell_env();
        let is_win = is_windows(&env);

        // Read user configuration so that `cangjie_home` set in LSP
        // settings is picked up here (just like lsp_server_command_impl).
        // An empty stdLibPath is safe — IsInCjlibDir() returns false for
        // all files when the path is empty, enabling all features.
        let configuration =
            self.language_server_workspace_configuration(language_server_id, worktree)?;
        let mut cangjie_home = get_cangjie_home(&configuration, worktree).unwrap_or_default();

        // Emergency fallback: try std::env::var directly (may work even if
        // worktree.shell_env() doesn't have it in the WASM sandbox).
        if cangjie_home.is_empty() {
            if let Ok(home) = std::env::var("CANGJIE_HOME") {
                if !home.is_empty() {
                    cangjie_home = home;
                }
            }
        }

        let root_path = worktree.root_path();
        let sep = path_sep(is_win);
        let std_lib_path = if !cangjie_home.is_empty() {
            format!("{cangjie_home}{sep}lib")
        } else {
            // Last resort: try common SDK install paths
            let candidate = if is_win {
                std::env::var("USERPROFILE")
                    .ok()
                    .map(|p| format!("{p}\\.cjv\\toolchains\\lts-1.0.5"))
                    .filter(|p| std::fs::metadata(p).is_ok())
                    .map(|p| format!("{p}\\lib"))
            } else {
                std::env::var("HOME")
                    .ok()
                    .map(|p| format!("{p}/.cjv/toolchains/lts-1.0.5"))
                    .filter(|p| std::fs::metadata(p).is_ok())
                    .map(|p| format!("{p}/lib"))
            };
            candidate.unwrap_or_default()
        };

        let target_lib = format!("{root_path}{sep}.cache{sep}lsp");

        // Do NOT send multiModuleOption.  When null, the server falls
        // back to populating moduleInfoMap from Zed's rootUri (which
        // goes through the same URI::Resolve pipeline as didOpen URIs,
        // so paths stay consistent).  Sending non-null keys breaks
        // Windows drive-letter handling in the server's URI parser.
        let _workspace_uri = format!("file:///{}", root_path.replace('\\', "/"));
        let _multi_module = build_multi_module(worktree, &root_path, sep, &_workspace_uri);

        Ok(Some(json!({
            "stdLibPathOption": std_lib_path,
            "modulesHomeOption": cangjie_home,
            "targetLib": target_lib,
            "telemetryOption": false,
            "extensionPath": "",
            "conditionCompileOption": {},
            "singleConditionCompileOption": {},
            "conditionCompilePaths": [],
        })))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> zed::Result<Option<Value>> {
        LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .map(|lsp_settings| lsp_settings.settings)
            .map_err(|err| format!("Failed to get LSP settings: {err}"))
    }

    fn label_for_completion(
        &self,
        _language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        let label = &completion.label;
        let detail = completion.detail.as_deref().unwrap_or("");

        completion.kind.map(|kind| match kind {
            CompletionKind::Variable | CompletionKind::Field | CompletionKind::Constant => {
                let code = if detail.is_empty() {
                    label.clone()
                } else {
                    format!("{label}: {detail}")
                };
                CodeLabel {
                    spans: vec![
                        CodeLabelSpan::code_range(0..label.len()),
                        if detail.is_empty() {
                            CodeLabelSpan::literal("".to_string(), None)
                        } else {
                            CodeLabelSpan::literal(format!(": {detail}"), None)
                        },
                    ],
                    code,
                    filter_range: (0..label.len()).into(),
                }
            }
            CompletionKind::Method | CompletionKind::Constructor => {
                let code = if detail.is_empty() {
                    format!("{label}()")
                } else {
                    format!("{label}{detail}")
                };
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(0..label.len())],
                    code,
                    filter_range: (0..label.len()).into(),
                }
            }
            CompletionKind::Class => {
                let code = format!("class {label}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(6..code.len())],
                    code,
                    filter_range: (0..label.len()).into(),
                }
            }
            CompletionKind::Interface => {
                let code = format!("interface {label}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(10..code.len())],
                    code,
                    filter_range: (0..label.len()).into(),
                }
            }
            CompletionKind::Enum => {
                let code = format!("enum {label}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(5..code.len())],
                    code,
                    filter_range: (0..label.len()).into(),
                }
            }
            CompletionKind::Keyword | CompletionKind::Snippet => CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..label.len())],
                filter_range: (0..label.len()).into(),
                code: label.clone(),
            },
            _ => CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..label.len())],
                filter_range: (0..label.len()).into(),
                code: label.clone(),
            },
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        let name = &symbol.name;
        let label = match symbol.kind {
            SymbolKind::Class => {
                let code = format!("class {name} {{}}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(6..6 + name.len())],
                    filter_range: (6..6 + name.len()).into(),
                    code,
                }
            }
            SymbolKind::Interface => {
                let code = format!("interface {name} {{}}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(10..10 + name.len())],
                    filter_range: (10..10 + name.len()).into(),
                    code,
                }
            }
            SymbolKind::Enum => {
                let code = format!("enum {name} {{}}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(5..5 + name.len())],
                    filter_range: (5..5 + name.len()).into(),
                    code,
                }
            }
            SymbolKind::Method | SymbolKind::Function => {
                let code = format!("func {name}() {{}}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(5..5 + name.len())],
                    filter_range: (5..5 + name.len()).into(),
                    code,
                }
            }
            SymbolKind::Variable => {
                let code = format!("let {name}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(4..4 + name.len())],
                    filter_range: (4..4 + name.len()).into(),
                    code,
                }
            }
            SymbolKind::Constant => {
                let code = format!("const {name}");
                CodeLabel {
                    spans: vec![CodeLabelSpan::code_range(6..6 + name.len())],
                    filter_range: (6..6 + name.len()).into(),
                    code,
                }
            }
            _ => CodeLabel {
                spans: vec![CodeLabelSpan::code_range(0..name.len())],
                filter_range: (0..name.len()).into(),
                code: name.clone(),
            },
        };
        Some(label)
    }
}
