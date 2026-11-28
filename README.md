# Zed extension for TypeScript 7 Language Server

This extension integrates TypeScript 7 with Zed, providing language server support for TypeScript, TSX, JavaScript, and JSX.

## Installation

1. Install Rust with `rustup` and clone this repository. Zed uses the Rust toolchain to compile the dev extension to WebAssembly.
2. In Zed, run `zed: install dev extension` and select the repository root containing `extension.toml`. [Developing extensions](https://zed.dev/docs/extensions/developing-extensions#developing-an-extension-locally)

Zed identifies both the extension and its language server as `tsgo-ls`.

## Configuration

Merge this preset into your Zed `settings.json` or your project's `.zed/settings.json`. It selects `tsgo-ls` for JavaScript and TypeScript, leaves other eligible servers such as ESLint available, and uses the TypeScript server for formatting, semantic highlighting, folding, and the outline. [Zed language settings](https://zed.dev/docs/configuring-languages)

```json
{
  "code_lens": "on",
  "languages": {
    "TypeScript": {
      "language_servers": ["tsgo-ls", "!vtsls", "!typescript-language-server", "..."],
      "formatter": { "language_server": { "name": "tsgo-ls" } },
      "semantic_tokens": "combined",
      "document_folding_ranges": "on",
      "document_symbols": "on",
      "inlay_hints": { "enabled": true }
    },
    "TSX": {
      "language_servers": ["tsgo-ls", "!vtsls", "!typescript-language-server", "..."],
      "formatter": { "language_server": { "name": "tsgo-ls" } },
      "semantic_tokens": "combined",
      "document_folding_ranges": "on",
      "document_symbols": "on",
      "inlay_hints": { "enabled": true }
    },
    "JavaScript": {
      "language_servers": ["tsgo-ls", "!vtsls", "!typescript-language-server", "..."],
      "formatter": { "language_server": { "name": "tsgo-ls" } },
      "semantic_tokens": "combined",
      "document_folding_ranges": "on",
      "document_symbols": "on",
      "inlay_hints": { "enabled": true }
    },
    "JSX": {
      "language_servers": ["tsgo-ls", "!vtsls", "!typescript-language-server", "..."],
      "formatter": { "language_server": { "name": "tsgo-ls" } },
      "semantic_tokens": "combined",
      "document_folding_ranges": "on",
      "document_symbols": "on",
      "inlay_hints": { "enabled": true }
    }
  }
}
```

### Inlay hints, CodeLens, and navigation

The extension enables parameter names for literal arguments, inferred variable type hints, and reference and implementation CodeLenses by default. Zed controls their visibility through `inlay_hints.enabled` and `code_lens`. Click a CodeLens to see its locations.

Set server preferences under `lsp.tsgo-ls.settings`. TypeScript applies the `editor`, `javascript`, `typescript`, and `js/ts` sections in that order. The extension puts its defaults in `editor`. This example requests parameter names for all arguments, disables variable type hints and reference lenses, and makes Go to Definition prefer original source when available. [TypeScript settings](docs/compability.md#workspace-preferences)

```json
{
  "lsp": {
    "tsgo-ls": {
      "settings": {
        "js/ts": {
          "inlayHints": {
            "parameterNames": { "enabled": "all" },
            "variableTypes": { "enabled": false }
          },
          "referencesCodeLens": { "enabled": false },
          "unstable": { "preferGoToSourceDefinition": true }
        }
      }
    }
  }
}
```

### Source actions on format

Add `code_actions_on_format` to a language to run source actions when you format it, including on save if `format_on_save` is enabled. This example organizes imports in TypeScript with TypeScript 7.0.2.

```json
{
  "languages": {
    "TypeScript": {
      "format_on_save": "on",
      "code_actions_on_format": { "source.organizeImports": true }
    }
  }
}
```

TypeScript 7.0.2 provides `source.organizeImports`, `source.removeUnusedImports`, `source.sortImports`, and `source.fixAll`. TypeScript 7.1 Beta adds `.ts` to those four kinds, for example `source.organizeImports.ts`. [TypeScript 7.0.2 actions](https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/lsp/server.go#L1152-L1160) · [TypeScript 7.1 Beta actions](https://github.com/microsoft/TypeScript/blob/v7.1-beta/tsc/internal/lsp/server.go#L364-L371)

## Development checks

```sh
rustup target add wasm32-wasip2
cargo fmt --all -- --check
cargo test --locked
node --test tests/highlight_proxy.test.cjs
cargo clippy --locked --all-targets -- -D warnings
cargo clippy --locked --target wasm32-wasip2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2
```
