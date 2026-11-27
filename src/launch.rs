mod highlight_proxy;

use zed_extension_api as z;

pub(super) fn command(
    language_server_id: &z::LanguageServerId,
    worktree: &z::Worktree,
) -> z::Result<z::Command> {
    let executable = crate::typescript::executable(language_server_id)?;
    let settings = z::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
    let (arguments, environment) = settings
        .binary
        .map_or_default(|binary| (binary.arguments, binary.env));

    let command = z::Command {
        command: executable
            .to_string_lossy()
            .into_owned(),
        args: arguments.unwrap_or_else(|| vec!["--lsp".into(), "--stdio".into()]),
        env: environment
            .unwrap_or_default()
            .into_iter()
            .collect(),
    };
    Ok(self::highlight_proxy::wrap(command))
}
