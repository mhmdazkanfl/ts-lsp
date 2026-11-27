use std::io::Write;
use std::path::{Path, PathBuf};

use zed_extension_api as z;

const PROXY: &[u8] = include_bytes!("highlight_proxy.cjs");

pub fn with_workaround(mut command: z::Command) -> z::Command {
    if !command
        .args
        .iter()
        .any(|arg| arg == "--lsp")
        || !command
            .args
            .iter()
            .any(|arg| arg == "--stdio")
    {
        return command;
    }

    match proxy_launch(&command.command) {
        Ok((node, script, binary)) => {
            command.command = node;
            command.args.splice(
                0..0,
                [
                    script.to_string_lossy().into_owned(),
                    binary.to_string_lossy().into_owned(),
                ],
            );
        }
        Err(error) => {
            eprintln!("Starting TypeScript without its document highlight proxy: {error}");
        }
    }
    command
}

fn proxy_launch(binary: &str) -> z::Result<(String, PathBuf, PathBuf)> {
    let node = z::node_binary_path()?;
    let directory = std::env::current_dir().map_err(|error| error.to_string())?;
    let script = install_proxy(&directory).map_err(|error| error.to_string())?;
    Ok((node, script, directory.join(binary)))
}

fn install_proxy(directory: &Path) -> std::io::Result<PathBuf> {
    let path = Path::new("highlight_proxy.cjs");
    let destination = directory.join(path);
    if !std::fs::read(path).is_ok_and(|contents| contents == PROXY) {
        // Keep the temporary file and its destination under the same WASI preopen.
        let (mut file, temporary) = tempfile::NamedTempFile::new_in(".")?.into_parts();
        file.write_all(PROXY)?;
        drop(file);
        temporary.persist(&destination)?;
    }
    Ok(destination)
}
