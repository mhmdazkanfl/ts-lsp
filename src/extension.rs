use zed_extension_api as z;

pub struct TsgoLs;

impl z::Extension for TsgoLs {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &z::LanguageServerId,
        _worktree: &z::Worktree,
    ) -> z::Result<z::Command> {
        let (os, arch) = z::current_platform();
        let binary = crate::typescript::ensure_installed(os, arch, |status| {
            z::set_language_server_installation_status(language_server_id, &status);
        })?;

        Ok(z::Command {
            command: binary.to_string_lossy().into_owned(),
            args: vec!["--lsp".into(), "--stdio".into()],
            env: Vec::default(),
        })
    }
}
