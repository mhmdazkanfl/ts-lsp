mod configuration;
mod launch;
mod typescript;

use zed_extension_api as z;

struct TsgoLs;

impl z::Extension for TsgoLs {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &z::LanguageServerId,
        worktree: &z::Worktree,
    ) -> z::Result<z::Command> {
        crate::launch::command(language_server_id, worktree)
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &z::LanguageServerId,
        _worktree: &z::Worktree,
    ) -> z::Result<Option<z::serde_json::Value>> {
        Ok(Some(
            crate::configuration::initialization_options(),
        ))
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &z::LanguageServerId,
        worktree: &z::Worktree,
    ) -> z::Result<Option<z::serde_json::Value>> {
        crate::configuration::workspace_configuration(language_server_id, worktree).map(Some)
    }
}

z::register_extension!(TsgoLs);
