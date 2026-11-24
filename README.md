# Zed extension for TypeScript 7 Language Server

This extension is being developed to bring Microsoft's Go-based TypeScript 7 language server to TypeScript, TSX, JavaScript, and JSX in Zed.

## Installation

1. Install Zed, or build it from source using the platform-specific instructions in the Zed repository. [Developing Zed](https://github.com/zed-industries/zed#developing-zed)
2. Install Rust with `rustup` and clone this repository. Zed uses the Rust toolchain to compile the dev extension to WebAssembly.
3. In Zed, run `zed: install dev extension` and select the repository root containing `extension.toml`. [Developing extensions](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)

Zed identifies both the extension and its language server as `tsgo-ls`.

## Configuration

TBA

## Development checks

### Check the source

Run these commands from the extension repository root.

```sh
rustup target add wasm32-wasip2
cargo fmt --all -- --check
cargo clippy --locked --target wasm32-wasip2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2
```

To check `extension.toml` against the bundled schema with the Tombi CLI, run `tombi lint --offline extension.toml`. [Manifest validation](docs/extension-manifest.md#enable-autocomplete)

### Test in Zed

1. Install the checkout as a dev extension and confirm that **TypeScript 7 Language Server** appears on the Extensions page.
2. Use **Rebuild** on the extension after changing Rust code or the manifest. Run `zed: open log` to check for build or load errors. [Developing extensions](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)
3. Once server startup is implemented, set `language_servers` to `["tsgo-ls"]` for TypeScript, TSX, JavaScript, and JSX in a test project's `.zed/settings.json`. [Choosing language servers](https://zed.dev/docs/configuring-languages#choosing-language-servers)
4. Open a file of each type. Run `dev: open language server logs` to confirm that `tsgo-ls` initializes, then verify diagnostics, completions, and go to definition. [Zed actions](https://zed.dev/docs/all-actions)

### Validate a release package

Build Zed's packaging CLI from a Zed source checkout with `cargo build --locked --release -p extension_cli`. With `zed-extension` on your PATH, run this from the extension repository root to validate the release manifest and build the package. [Zed packaging CLI](https://github.com/zed-industries/zed/blob/main/crates/extension_cli/src/main.rs)

```sh
zed-extension --source-dir . --output-dir target/zed-package --scratch-dir target/zed-package-build
```
