use zed_extension_api as z;

const PROXY: &str = concat!(
    include_str!("highlight_proxy.cjs"),
    "\nrun();"
);

pub(super) fn wrap(mut command: z::Command) -> z::Command {
    let uses_stdio_lsp = ["--lsp", "--stdio"]
        .into_iter()
        .all(|flag| {
            command
                .args
                .iter()
                .any(|arg| arg == flag)
        });
    if uses_stdio_lsp && let Err(error) = attach_highlight_proxy(&mut command) {
        eprintln!("Starting TypeScript without its document highlight proxy: {error}");
    }
    command
}

// Resolve paths before changing the command so any setup error leaves a usable direct launch.
fn attach_highlight_proxy(command: &mut z::Command) -> z::Result<()> {
    let node = z::node_binary_path()?;
    let directory = std::env::current_dir()
        .map_err(|error| format!("Failed to resolve the extension directory: {error}"))?;
    let executable = directory.join(&command.command);

    command.command = node;
    command.args.splice(
        0..0,
        [
            "--input-type=commonjs".into(),
            "--eval".into(),
            PROXY.into(),
            "--".into(),
            executable
                .to_string_lossy()
                .into_owned(),
        ],
    );
    Ok(())
}
