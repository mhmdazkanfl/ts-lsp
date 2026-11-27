use zed_extension_api as z;

const PROXY_NAME: &str = "highlight_proxy.cjs";
const PROXY: &[u8] = include_bytes!("highlight_proxy.cjs");

pub(super) fn wrap(mut command: z::Command) -> z::Command {
    let uses_stdio_lsp = ["--lsp", "--stdio"].iter().all(|flag| {
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
    let directory = std::env::current_dir().map_err(|error| error.to_string())?;
    let script = install_proxy(&directory).map_err(|error| error.to_string())?;
    let executable = directory.join(&command.command);

    command.command = node;
    command.args.splice(
        0..0,
        [
            script.to_string_lossy().into_owned(),
            executable
                .to_string_lossy()
                .into_owned(),
        ],
    );
    Ok(())
}

fn install_proxy(directory: &std::path::Path) -> std::io::Result<std::path::PathBuf> {
    let destination = directory.join(PROXY_NAME);
    if !std::fs::read(&destination).is_ok_and(|contents| contents == PROXY) {
        // Keep the temporary file and its destination under the same WASI preopen.
        let (mut file, temporary) = tempfile::NamedTempFile::new_in(directory)?.into_parts();
        std::io::Write::write_all(&mut file, PROXY)?;
        drop(file);
        temporary.persist(&destination)?;
    }
    Ok(destination)
}

#[cfg(test)]
mod tests {
    #[test]
    fn installs_the_embedded_proxy_over_a_stale_copy_in_the_extension_directory() {
        let directory = tempfile::tempdir().unwrap();
        let script = directory.path().join(super::PROXY_NAME);
        std::fs::write(&script, b"stale script").unwrap();

        assert_eq!(
            super::install_proxy(directory.path()).unwrap(),
            script
        );
        assert_eq!(
            std::fs::read(script).unwrap(),
            super::PROXY
        );
    }
}
