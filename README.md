# Zed extension for TypeScript 7 Language Server

This extension integrates TypeScript 7 with Zed, providing language server support for TypeScript, TSX, JavaScript, and JSX.

## Installation

1. Install Rust with `rustup` and clone this repository. Zed uses the Rust toolchain to compile the dev extension to WebAssembly.
2. In Zed, run `zed: install dev extension` and select the repository root containing `extension.toml`. [Developing extensions](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)

Zed identifies both the extension and its language server as `tsgo-ls`.

## Configuration

TBA

## Development checks

```sh
rustup target add wasm32-wasip2
cargo fmt --all -- --check
cargo test --locked
cargo clippy --locked --target wasm32-wasip2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2
```
