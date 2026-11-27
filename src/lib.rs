mod configuration;
mod labels;
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

    fn label_for_completion(
        &self,
        _language_server_id: &z::LanguageServerId,
        completion: z::lsp::Completion,
    ) -> Option<z::CodeLabel> {
        crate::labels::completion(completion)
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &z::LanguageServerId,
        symbol: z::lsp::Symbol,
    ) -> Option<z::CodeLabel> {
        crate::labels::symbol(symbol)
    }
}

z::register_extension!(TsgoLs);
