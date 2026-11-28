use zed_extension_api as z;

const PROXY_NAME: &str = "highlight_proxy.cjs";
const PROXY: &[u8] = include_bytes!("highlight_proxy.cjs");

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
    // WASI treats Windows drive-letter paths as relative. Use Zed's `.` preopen for file I/O.
    install_proxy(std::path::Path::new(".")).map_err(|error| {
        format!(
            "Failed to install {PROXY_NAME} in {}: {error}",
            directory.display()
        )
    })?;
    let script = directory.join(PROXY_NAME);
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

fn install_proxy(directory: &std::path::Path) -> std::io::Result<()> {
    let destination = directory.join(PROXY_NAME);
    if std::fs::read(&destination).is_ok_and(|contents| contents == PROXY) {
        return Ok(());
    }

    let (path, mut temporary) = loop {
        let path = directory.join(format!(
            ".{PROXY_NAME}.{:016x}.tmp",
            fastrand::u64(..)
        ));
        match std::fs::File::create_new(&path) {
            Ok(file) => break (path, file),
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {}
            Err(error) => return Err(error),
        }
    };
    let written = std::io::Write::write_all(&mut temporary, PROXY);
    drop(temporary);
    if let Err(error) = written.and_then(|()| std::fs::rename(&path, &destination)) {
        let _ = std::fs::remove_file(path);
        return Err(error);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn installs_the_embedded_proxy_over_a_stale_copy_in_the_extension_directory() {
        let directory = tempfile::tempdir().unwrap();
        let script = directory.path().join(super::PROXY_NAME);
        std::fs::write(&script, b"stale script").unwrap();

        super::install_proxy(directory.path()).unwrap();
        assert_eq!(
            std::fs::read(script).unwrap(),
            super::PROXY
        );
    }
}
