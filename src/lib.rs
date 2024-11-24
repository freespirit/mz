use zed_extension_api as zed;

struct MojoLspExtension {
    // ... state
}

impl zed::Extension for MojoLspExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        Ok(zed::Command {
            command: "magic".to_string(),
            args: vec!["run".to_string(), "mojo-lsp-server".to_string()],
            env: vec![],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        let settings = zed::settings::LspSettings::for_worktree("mojo-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        Ok(Some(settings))
    }
}

zed::register_extension!(MojoLspExtension);
