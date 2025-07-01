use zed_extension_api::{self as zed};

// The name used to configure the lsp server in the Zed settings
const LSP_SERVER_ID: &str = "mojo-lsp";

struct MojoExtension {
    command: String,
    args: Vec<String>,
}

impl zed::Extension for MojoExtension {
    fn new() -> Self {
        Self {
            command: "pixi".to_string(),
            args: vec!["run".to_string(), "mojo-lsp-server".to_string()],
        }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let settings = zed::settings::LspSettings::for_worktree(LSP_SERVER_ID, _worktree);

        if let Ok(lsp_settings) = settings {
            if let Some(binary) = lsp_settings.binary {
                let env: Vec<(String, String)> = match binary.env {
                    Some(m) => m.into_iter().collect(),
                    None => Vec::new(),
                };

                return Ok(zed::Command {
                    command: binary.path.unwrap_or(self.command.clone()),
                    args: binary.arguments.unwrap_or(self.args.clone()),
                    env,
                });
            }
        }

        Ok(zed::Command {
            command: self.command.clone(),
            args: self.args.clone(),
            env: vec![],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree(LSP_SERVER_ID, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(MojoExtension);
