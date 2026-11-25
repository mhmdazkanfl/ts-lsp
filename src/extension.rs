use zed_extension_api as z;

pub struct TsgoLs;

impl z::Extension for TsgoLs {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &z::LanguageServerId,
        worktree: &z::Worktree,
    ) -> z::Result<z::Command> {
        let (os, arch) = z::current_platform();
        let binary = crate::typescript::ensure_installed(os, arch, |status| {
            z::set_language_server_installation_status(language_server_id, &status);
        })?;
        let lsp_settings =
            z::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;

        let (arguments, env) = lsp_settings
            .binary
            .map_or((None, None), |binary| {
                (binary.arguments, binary.env)
            });

        Ok(z::Command {
            command: binary.to_string_lossy().into_owned(),
            args: arguments.unwrap_or_else(|| vec!["--lsp".into(), "--stdio".into()]),
            env: env
                .unwrap_or_default()
                .into_iter()
                .collect(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &z::LanguageServerId,
        _worktree: &z::Worktree,
    ) -> z::Result<Option<z::serde_json::Value>> {
        Ok(Some(z::serde_json::json!({
            "codeLensShowLocationsCommandName": "editor.action.showReferences",
        })))
    }
}
