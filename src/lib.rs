use zed_extension_api as zed;

struct CangjieExtension;

impl zed::Extension for CangjieExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let env = worktree.shell_env();

        // 1. Try PATH first (SDK installer sets this automatically)
        if let Some(server_path) = worktree.which("LSPServer.exe") {
            return Ok(zed::Command {
                command: server_path,
                args: vec![],
                env,
            });
        }

        // 2. Fallback: check CANGJIE_HOME env var
        if let Some(path) = env
            .iter()
            .find_map(|(k, v)| {
                if k.eq_ignore_ascii_case("CANGJIE_HOME") {
                    Some(format!("{}\\bin\\LSPServer.exe", v))
                } else {
                    None
                }
            })
        {
            return Ok(zed::Command {
                command: path,
                args: vec![],
                env,
            });
        }

        Err("LSPServer.exe not found.\n\n\
            Install the Cangjie SDK from https://cangjie-lang.cn/download/1.0.5 \n\
            and ensure the installation bin directory is added to your PATH. \n\
            A system restart may be required if you used the EXE installer."
            .to_string())
    }
}

zed::register_extension!(CangjieExtension);
