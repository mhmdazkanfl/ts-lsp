use zed_extension_api as z;

const PROXY: &str = concat!(
    include_str!("highlight_proxy.cjs"),
    "\nrun();"
);

pub(super) struct Runtime {
    node: String,
    directory: std::path::PathBuf,
}

impl Runtime {
    pub(super) fn resolve() -> z::Result<Self> {
        let node = z::node_binary_path()?;
        let directory = std::env::current_dir()
            .map_err(|error| format!("Failed to resolve the extension directory: {error}"))?;
        Ok(Self {
            node,
            directory,
        })
    }
}

pub(super) fn wrap(
    mut command: z::Command,
    resolve_runtime: impl FnOnce() -> z::Result<Runtime>,
) -> z::Command {
    let uses_stdio_lsp = ["--lsp", "--stdio"]
        .into_iter()
        .all(|flag| {
            command
                .args
                .iter()
                .any(|arg| arg == flag)
        });
    if !uses_stdio_lsp {
        return command;
    }
    match resolve_runtime() {
        Ok(runtime) => attach_highlight_proxy(&mut command, runtime),
        Err(error) => {
            eprintln!("Starting TypeScript without its document highlight proxy: {error}");
        }
    }
    command
}

fn attach_highlight_proxy(command: &mut z::Command, runtime: Runtime) {
    let executable = runtime.directory.join(&command.command);

    command.command = runtime.node;
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
}

#[cfg(test)]
mod tests {
    use std::io::Write as _;
    use std::path::PathBuf;
    use std::process::{Child, Command, Output, Stdio};
    use std::time::{Duration, Instant};

    use zed_extension_api as z;

    use super::{Runtime, wrap};

    fn server_command(args: &[&str]) -> z::Command {
        z::Command {
            command: "node_modules/typescript/tsc".into(),
            args: args
                .iter()
                .map(|arg| (*arg).into())
                .collect(),
            env: vec![(
                "HIGHLIGHT_PROXY_TEST".into(),
                "fixture".into(),
            )],
        }
    }

    fn runtime() -> Runtime {
        Runtime {
            node: "zed-node".into(),
            directory: PathBuf::from("extension"),
        }
    }

    fn assert_launch_matches(actual: &z::Command, expected: &z::Command) {
        assert_eq!(actual.command, expected.command);
        assert_eq!(actual.args, expected.args);
        assert_eq!(actual.env, expected.env);
    }

    #[test]
    fn a_server_without_lsp_starts_directly() {
        let args = ["--stdio"];

        let launch = wrap(server_command(&args), || Ok(runtime()));

        assert_launch_matches(&launch, &server_command(&args));
    }

    #[test]
    fn a_similar_argument_does_not_enable_the_stdio_proxy() {
        let args = ["--lsp", "--stdio=false"];

        let launch = wrap(server_command(&args), || Ok(runtime()));

        assert_launch_matches(&launch, &server_command(&args));
    }

    #[test]
    fn a_runtime_lookup_failure_keeps_the_direct_server_launch() {
        let args = ["--lsp", "--stdio", "--locale", "en"];

        let launch = wrap(server_command(&args), || {
            Err("Node is unavailable".into())
        });

        assert_launch_matches(&launch, &server_command(&args));
    }

    fn wait_for_proxy(mut child: Child) -> Output {
        let deadline = Instant::now() + Duration::from_secs(5);
        while child.try_wait().unwrap().is_none() {
            if Instant::now() >= deadline {
                let _ = child.kill();
                let output = child.wait_with_output().unwrap();
                panic!(
                    "The highlight proxy did not exit: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        child.wait_with_output().unwrap()
    }

    #[test]
    #[ignore = "requires Node.js on PATH"]
    fn the_generated_launch_runs_the_embedded_proxy_and_preserves_server_options() {
        let node = Command::new("node")
            .args(["-p", "process.execPath"])
            .output()
            .expect("Node.js must be available on PATH");
        assert!(node.status.success());
        let node = String::from_utf8(node.stdout)
            .unwrap()
            .trim()
            .to_owned();
        let executable = std::path::Path::new(&node);
        let directory = executable
            .parent()
            .unwrap()
            .to_path_buf();
        let worktree = tempfile::tempdir().unwrap();
        let server = r#"
            const assert = require("node:assert/strict");
            assert.deepEqual(process.argv.slice(1), ["--lsp", "--stdio", "with spaces"]);
            assert.equal(process.env.HIGHLIGHT_PROXY_TEST, "fixture");
            process.stdin.pipe(process.stdout);
        "#;
        let mut direct = server_command(&[
            "-e",
            server,
            "--",
            "--lsp",
            "--stdio",
            "with spaces",
        ]);
        direct.command = executable
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let launch = wrap(direct, || {
            Ok(Runtime {
                node,
                directory,
            })
        });
        let body = r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/documentHighlight","params":{"textDocument":{"uri":"file:///work/(entry).ts"},"position":{"line":0,"character":6}}}"#;
        let request = format!(
            "Content-Length: {}\r\n\r\n{body}",
            body.len()
        );
        let mut sut = Command::new(launch.command)
            .args(launch.args)
            .envs(launch.env)
            .current_dir(worktree.path())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();

        sut.stdin
            .take()
            .unwrap()
            .write_all(request.as_bytes())
            .unwrap();
        let output = wait_for_proxy(sut);

        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let text = String::from_utf8(output.stdout).unwrap();
        let (_, response_body) = text.split_once("\r\n\r\n").unwrap();
        let response: z::serde_json::Value = z::serde_json::from_str(response_body).unwrap();
        assert_eq!(
            response["params"]["textDocument"]["uri"],
            "file:///work/%28entry%29.ts"
        );
    }
}
