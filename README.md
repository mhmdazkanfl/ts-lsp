# tsgo: Native TypeScript Compiler Integration for Zed

This extension integrates TypeScript v7's native Go-based compiler and language server into the Zed editor, delivering enhanced performance and efficiency for TypeScript development.

## 🚀 Why the native compiler?

With TypeScript 7, Microsoft shipped the TypeScript compiler as a native version written in Go, with significant performance improvements:

- **Faster Compilation**: Achieves up to 10x speed improvements in large projects.
- **Reduced Memory Usage**: Optimized memory handling in native execution.
- **Improved Editor Performance**: Faster IntelliSense and language services.
- **Scalability**: Better handling of large codebases.

> _Example Benchmarks_:
>
> - **VS Code**: 77.8s → 7.5s (10.4x speedup)
> - **Playwright**: 11.1s → 1.1s (10.1x speedup)
> - **TypeORM**: 17.5s → 1.3s (13.5x speedup)
>
> _Source: [Microsoft Developer Blog](https://devblogs.microsoft.com/typescript/typescript-native-port/)_

## 🛠 Installation

1. Install Rust with `rustup` and clone this repository.
2. In Zed, run `zed: install dev extension` and select the checkout containing `extension.toml`.

The dev extension uses the same `tsgo` extension ID and `typescript-ls` language server ID as upstream. Zed treats it as a development override of the published extension.

## ⚙️ Configuration

### Basic Setup

Enable `typescript-ls` in your Zed settings:

```jsonc
{
    "languages": {
        "TypeScript": {
            "language_servers": [
                "typescript-ls",
                "!vtsls",
                "!typescript-language-server",
                "...",
            ],
        },
        "TSX": {
            "language_servers": [
                "typescript-ls",
                "!vtsls",
                "!typescript-language-server",
                "...",
            ],
        },
    },
}
```

The setup above gives TypeScript and TSX a single TypeScript server while retaining other configured servers, such as linters. You can list `vtsls` alongside `typescript-ls` when you want both servers. They can report overlapping diagnostics or completion results.

```jsonc
{
    "languages": {
        "TypeScript": {
            "language_servers": [
                "typescript-ls",
                "vtsls",
                "!typescript-language-server",
                "...",
            ],
        },
        "TSX": {
            "language_servers": [
                "typescript-ls",
                "vtsls",
                "!typescript-language-server",
                "...",
            ],
        },
    },
}
```

### Advanced Configuration

#### Choosing the TypeScript binary

The extension uses `lsp.typescript-ls.binary.path` when configured. Otherwise it looks for a compatible project-installed `typescript` package and its matching native binary, including ancestor `node_modules` directories in a monorepo. The extension launches the native binary directly. Package layouts that expose TypeScript through `node_modules` are supported, including pnpm's isolated layout. For Yarn Plug'n'Play, set an explicit native binary path or use the managed install.

If no compatible project binary is available, the extension installs the `typescript` package into its own directory. It checks for the latest release on the first managed launch in a Zed session. If the registry check or update fails, it can reuse a valid managed install. An explicit version request reports an error if the requested version cannot be installed. Restart the language server after changing its binary settings or switching the project's TypeScript version.

A custom binary path can be absolute, relative to the worktree (such as `./tools/tsc`), or a command on the worktree's `PATH`. Omit `arguments` to use `--lsp --stdio`. Set `arguments` to replace the entire argument list. `env` supplies additional variables to the launched server.

```json
{
  "lsp": {
    "typescript-ls": {
      "binary": {
        "path": "./tools/tsc",
        "arguments": ["--lsp", "--stdio"],
        "env": { "GOMAXPROCS": "4" }
      }
    }
  }
}
```

#### Pinning a package version

Set `package_version` to pin TypeScript to a native release (7.0.0 or newer). The project binary is eligible if its version matches the pin. Set `prefer_project_binary` to `false` to always use the managed package unless you supplied `binary.path`.

```json
{
    "lsp": {
        "typescript-ls": {
            "settings": {
                "package_version": "7.0.2",
                "prefer_project_binary": false
            }
        }
    }
}
```

When an npm tag or range is used instead of an exact version, npm resolves it through the managed install. For an install error, check `zed: open log`. If a suitable project package is installed but the project lookup fails, launch Zed with `--foreground` to see the lookup error.

#### Inlay Hints

The extension sets defaults to make `typescript-ls` send back inlay hints when Zed has inlay hints enabled. To show them, enable Zed's `inlay_hints` setting:

```json
{
  "inlay_hints": {
    "enabled": true
  }
}
```

You can override the default inlay hint settings in your Zed settings:

```json
{
  "lsp": {
    "typescript-ls": {
      "settings": {
        "typescript": {
          "inlayHints": {
            "parameterNames": {
              "enabled": "none"
            },
            "parameterTypes": {
              "enabled": false
            },
            "variableTypes": {
              "enabled": false
            },
            "propertyDeclarationTypes": {
              "enabled": false
            },
            "functionLikeReturnTypes": {
              "enabled": false
            },
            "enumMemberValues": {
              "enabled": false
            }
          }
        }
      }
    }
  }
}
```

Use the same settings under `javascript` to override them for JavaScript.

#### Code Lens

The extension enables references and implementations code lenses by default. These show reference counts and implementation counts above functions, classes, and interfaces. To use them, enable Zed's `code_lens` setting:

```json
{
  "code_lens": "on"
}
```

You can override the default code lens settings in your Zed settings:

```json
{
  "lsp": {
    "typescript-ls": {
      "settings": {
        "typescript": {
          "implementationsCodeLens": {
            "enabled": false
          },
          "referencesCodeLens": {
            "enabled": false
          }
        }
      }
    }
  }
}
```

Use the same settings under `javascript` to override them for JavaScript.

#### Editor feature presets

For a minimal setup, use the single-server configuration above and leave Zed's editor settings at their defaults. For more language-server UI, merge the following settings with your existing configuration. `combined` overlays semantic tokens on Tree-sitter highlighting. Zed enables pull diagnostics and linked editing by default, so set those explicitly only if you have overridden them elsewhere.

```json
{
  "languages": {
    "TypeScript": { "semantic_tokens": "combined", "document_folding_ranges": "on" },
    "TSX": { "semantic_tokens": "combined", "document_folding_ranges": "on" },
    "JavaScript": { "semantic_tokens": "combined", "document_folding_ranges": "on" },
    "JSX": { "semantic_tokens": "combined", "document_folding_ranges": "on" }
  },
  "inlay_hints": { "enabled": true },
  "code_lens": "on",
  "linked_edits": true,
  "diagnostics": { "lsp_pull_diagnostics": { "enabled": true } }
}
```

Completion and workspace-symbol labels use TypeScript syntax highlighting for recognized symbol kinds. The extension keeps Zed's built-in presentation for other kinds.

[Zed's language-server settings](https://zed.dev/docs/configuring-languages#using-language-server-features)

### Zed client support

Zed's LSP client controls which protocol features surface in the editor. The extension API exposes server launch, configuration, and label hooks. LSP selection ranges, range-based semantic token requests, and TypeScript-specific actions such as Go to Source Definition require client support in Zed.

[Zed's extension API](https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/trait.Extension.html)

## Development checks

```sh
cargo clippy --locked --target wasm32-wasip2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2
node --test tests/project_binary.test.cjs
```
