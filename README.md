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

### Enable the language server

Use `typescript-ls` for the languages you want it to handle. This example disables the other TypeScript servers while keeping additional servers such as linters. To run `vtsls` alongside `typescript-ls`, replace `"!vtsls"` with `"vtsls"` for that language. Both servers may provide diagnostics and completions.

```json
{
  "languages": {
    "TypeScript": { "language_servers": ["typescript-ls", "!vtsls", "!typescript-language-server", "..."] },
    "TSX": { "language_servers": ["typescript-ls", "!vtsls", "!typescript-language-server", "..."] },
    "JavaScript": { "language_servers": ["typescript-ls", "!vtsls", "!typescript-language-server", "..."] },
    "JSX": { "language_servers": ["typescript-ls", "!vtsls", "!typescript-language-server", "..."] }
  }
}
```

### Full `lsp.typescript-ls` configuration

This reference covers the fork's launch settings and TypeScript **7.0.2** server options. All keys are optional. Copy the settings you want to override. Some values below illustrate a choice and differ from the server defaults. Use your Zed user `settings.json` or a project's `.zed/settings.json`.

```jsonc
{
  "lsp": {
    "typescript-ls": {
      "binary": {
        // Optional absolute or worktree-relative executable path.
        // "path": "./tools/tsc",
        // Include both arguments when setting binary.path to native tsc.
        "arguments": ["--lsp", "--stdio"],
        // Additional environment variables. Values must be strings.
        "env": {}
      },
      "initialization_options": {
        "codeLensShowLocationsCommandName": "editor.action.showReferences",
        "disablePushDiagnostics": false,
        "enableTelemetry": false,
        // 0 off, 1 trace, 2 debug, 3 info, 4 warning, 5 error.
        "logVerbosity": 3,
        // Fallback for clients without workspace/configuration. See notes below.
        "userPreferences": {}
      },
      "settings": {
        // Optional exact release, npm tag, or npm version range.
        // "package_version": "7.0.2",
        "prefer_project_binary": true,
        "editor": {},
        "javascript": {},
        "typescript": {},
        "js/ts": {
          // A filename, such as "tsconfig.editor.json". Empty uses normal discovery.
          "customConfigFileName": "",
          "autoClosingTags": { "enabled": true },
          "suggest": {
            "autoImports": true,
            "includeCompletionsForImportStatements": true,
            "includeAutomaticOptionalChainCompletions": true,
            "classMemberSnippets": { "enabled": true },
            "objectLiteralMethodSnippets": { "enabled": true },
            "jsdoc": { "enabled": true, "generateReturns": true }
          },
          "preferences": {
            "quoteStyle": "auto", // auto, single, double
            "jsxAttributeCompletionStyle": "auto", // auto, braces, none
            "importModuleSpecifier": "shortest", // shortest, relative, non-relative, project-relative
            "importModuleSpecifierEnding": "auto", // auto, minimal, index, js
            "autoImportSpecifierExcludeRegexes": [], // Array of regex strings
            "autoImportFileExcludePatterns": [], // Array of file glob strings
            "autoImportEntrypointDirectorySearch": false,
            "preferTypeOnlyAutoImports": false,
            "useAliasesForRenames": true,
            "organizeImports": {
              "sort": "auto", // auto, ordinal, ordinalIgnoreCase, natural, naturalIgnoreCase
              "caseSensitivity": "auto", // auto, caseSensitive, caseInsensitive
              "unicodeCollation": "ordinal", // ordinal, unicode
              "locale": "en", // Parsed for compatibility. Sorting is locale-independent in 7.0.2.
              "numericCollation": false,
              "accentCollation": true,
              "caseFirst": "default", // default, lower, upper
              "typeOrder": "auto" // auto, first, inline, last
            }
          },
          "format": {
            "enabled": true,
            "baseIndentSize": 0,
            "indentSize": 4,
            "tabSize": 4,
            "newLineCharacter": "\n",
            "convertTabsToSpaces": true,
            "indentStyle": "smart", // none, block, smart
            "trimTrailingWhitespace": true,
            "insertSpaceAfterCommaDelimiter": true,
            "insertSpaceAfterSemicolonInForStatements": true,
            "insertSpaceBeforeAndAfterBinaryOperators": true,
            "insertSpaceAfterConstructor": false,
            "insertSpaceAfterKeywordsInControlFlowStatements": true,
            "insertSpaceAfterFunctionKeywordForAnonymousFunctions": false,
            "insertSpaceAfterOpeningAndBeforeClosingNonemptyParenthesis": false,
            "insertSpaceAfterOpeningAndBeforeClosingNonemptyBrackets": false,
            "insertSpaceAfterOpeningAndBeforeClosingNonemptyBraces": true,
            "insertSpaceAfterOpeningAndBeforeClosingEmptyBraces": false,
            "insertSpaceAfterOpeningAndBeforeClosingTemplateStringBraces": false,
            "insertSpaceAfterOpeningAndBeforeClosingJsxExpressionBraces": false,
            "insertSpaceAfterTypeAssertion": false,
            "insertSpaceBeforeFunctionParenthesis": false,
            "placeOpenBraceOnNewLineForFunctions": false,
            "placeOpenBraceOnNewLineForControlBlocks": false,
            "insertSpaceBeforeTypeAnnotation": false,
            "indentMultiLineObjectLiteralBeginningOnBlankLine": false,
            "semicolons": "ignore", // ignore, insert, remove
            "indentSwitchCase": true
          },
          "inlayHints": {
            "parameterNames": {
              "enabled": "all", // none, literals, all
              "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": { "enabled": true },
            "variableTypes": { "enabled": true, "suppressWhenTypeMatchesName": false },
            "propertyDeclarationTypes": { "enabled": true },
            "functionLikeReturnTypes": { "enabled": true },
            "enumMemberValues": { "enabled": true }
          },
          "referencesCodeLens": { "enabled": true, "showOnAllFunctions": true },
          "implementationsCodeLens": {
            "enabled": true,
            "showOnAllClassMethods": true,
            "showOnInterfaceMethods": true
          },
          "workspaceSymbols": { "excludeLibrarySymbols": true },
          "validate": { "enabled": true },
          "reportStyleChecksAsWarnings": true,
          // Legacy ATA preference. The explicit tsserver setting below takes precedence.
          "disableAutomaticTypeAcquisition": false,
          "tsserver": { "automaticTypeAcquisition": { "enabled": true } },
          "unstable": {
            "maximumHoverLength": 500,
            "allowRenameOfImportPath": true,
            "preferGoToSourceDefinition": false,
            // These fields are accepted by the 7.0.2 preference parser.
            // They have no independent effect in its LSP implementation.
            "lazyConfiguredProjectsFromExternalProject": false,
            "allowTextChangesInNewFiles": false,
            "provideRefactorNotApplicableReason": true,
            "disableSuggestions": false,
            "disableLineTextInReferences": true,
            "displayPartsForJSDoc": true
          }
        }
      }
    }
  }
}
```

#### Launch and startup options

| Setting | How the fork uses it |
| --- | --- |
| `binary.path` | Takes precedence over package selection. Zed launches this executable directly. Specify `binary.arguments` alongside `binary.path` when pointing at native `tsc`. |
| `binary.arguments` | The fork supplies `["--lsp", "--stdio"]` when it selects the binary and no arguments are set. An explicit `binary.path` uses the arguments you give Zed. You can append `--pprofDir` followed by an output directory. |
| `binary.env` | Adds string-valued environment variables to the launched server, such as `"GOMAXPROCS": "4"`. |
| `settings.prefer_project_binary` | Defaults to `true`. Looks for a native TypeScript installation in the worktree or an ancestor's `node_modules`, including pnpm's isolated layout. Set it to `false` to use the managed install. Yarn Plug'n'Play can use a custom path or the managed install. |
| `settings.package_version` | With no pin, the fork uses compatible project TypeScript or checks npm for a managed release. An exact pin must match the project version for the project binary to qualify. npm resolves tags and ranges through the managed install. A failed explicit version request reports an error. An unpinned launch can reuse a valid managed install when updating fails. |
| `initialization_options.codeLensShowLocationsCommandName` | Client command used by resolved references and implementations CodeLens. Keep the extension's default `editor.action.showReferences` for Zed. |
| `initialization_options.disablePushDiagnostics` | Disables server-pushed diagnostics when `true`. Pull diagnostics are controlled by Zed and `settings["js/ts"].validate.enabled`. |
| `initialization_options.enableTelemetry` | Enables the server's LSP telemetry events when `true`. Default `false`. |
| `initialization_options.logVerbosity` | Initial server logging level. Default `3` (info). |
| `initialization_options.userPreferences` | TypeScript reads this fallback object when the LSP client lacks workspace configuration support. With Zed, put preferences under `settings`. |

Restart the language server after changing its launch or initialization settings. Use `lsp.typescript-ls` for new settings. The fork also reads `lsp.tsgo` as a fallback for existing configurations.

Zed's per-server schema also accepts `binary.ignore_system_version`, `fetch.pre_release`, and `enable_lsp_tasks`. These fields have no effect on this fork's binary selection or on the TypeScript 7.0.2 server. Use `prefer_project_binary` and `package_version` for binary and release selection.

#### Workspace preference precedence

TypeScript 7.0.2 builds a shared preference set by applying `editor`, `javascript`, `typescript`, and the literal `js/ts` key in that order. Later sections override earlier sections. Use `js/ts` for settings that should take precedence over the fork's built-in inlay hint and CodeLens defaults, which it supplies under `javascript` and `typescript`.

- `editor` accepts the flat preference keys, such as `tabSize` and `convertTabsToSpaces`. The other three sections accept nested keys, flat keys, and an `unstable` object containing flat keys. Within each of those sections, `unstable` overrides top-level flat keys, and recognized nested settings take precedence over both.
- A custom `customConfigFileName` is a filename only, such as `tsconfig.editor.json`. The server falls back to `tsconfig.json` or `jsconfig.json` when it can't find the custom file. Set compiler options in the project config.
- `unstable.preferGoToSourceDefinition: true` makes the existing Go to Definition action prefer source files where the server can find them.
- Zed sends `tab_size`, `hard_tabs`, and `remove_trailing_whitespace_on_save` with formatting requests. TypeScript uses those request values for tab width, indent width, tabs versus spaces, and trailing-whitespace handling. To use TypeScript's formatter, set `"formatter": "language_server"` for the relevant languages.
- `autoClosingTags.enabled` controls the server's auto-insert handler. Any feature that needs an editor action still depends on Zed's client support.

<details>
<summary>Flat preference aliases and compatibility keys</summary>

Use the right-hand key as a literal name inside a section's `unstable` object or directly inside `editor`. Formatting fields use their leaf names, such as `semicolons` and `indentSize`, except `format.enabled`, whose flat key is `formatEnabled`.

| Nested key | Flat key |
| --- | --- |
| `preferences.quoteStyle` | `quotePreference` |
| `preferences.jsxAttributeCompletionStyle` | `jsxAttributeCompletionStyle` |
| `preferences.importModuleSpecifier` | `importModuleSpecifierPreference` |
| `preferences.importModuleSpecifierEnding` | `importModuleSpecifierEnding` |
| `preferences.autoImportSpecifierExcludeRegexes` | `autoImportSpecifierExcludeRegexes` |
| `preferences.autoImportFileExcludePatterns` | `autoImportFileExcludePatterns` |
| `preferences.autoImportEntrypointDirectorySearch` | `autoImportEntrypointDirectorySearch` |
| `preferences.preferTypeOnlyAutoImports` | `preferTypeOnlyAutoImports` |
| `preferences.useAliasesForRenames` | `providePrefixAndSuffixTextForRename` |
| `preferences.organizeImports.sort` | `organizeImportsSort` |
| `preferences.organizeImports.caseSensitivity` | `organizeImportsIgnoreCase` (`true` for case-insensitive, `false` for case-sensitive) |
| `preferences.organizeImports.unicodeCollation` | `organizeImportsCollation` |
| `preferences.organizeImports.locale` | `organizeImportsLocale` |
| `preferences.organizeImports.numericCollation` | `organizeImportsNumericCollation` |
| `preferences.organizeImports.accentCollation` | `organizeImportsAccentCollation` |
| `preferences.organizeImports.caseFirst` | `organizeImportsCaseFirst` |
| `preferences.organizeImports.typeOrder` | `organizeImportsTypeOrder` |
| `suggest.autoImports` | `includeCompletionsForModuleExports` |
| `suggest.includeCompletionsForImportStatements` | `includeCompletionsForImportStatements` |
| `suggest.includeAutomaticOptionalChainCompletions` | `includeAutomaticOptionalChainCompletions` |
| `suggest.classMemberSnippets.enabled` | `includeCompletionsWithClassMemberSnippets` |
| `suggest.objectLiteralMethodSnippets.enabled` | `includeCompletionsWithObjectLiteralMethodSnippets` |
| `suggest.jsdoc.enabled` | `completeJSDocs` |
| `suggest.jsdoc.generateReturns` | `generateReturnInDocTemplate` |
| `autoClosingTags.enabled` | `autoClosingTags` (boolean) |
| `inlayHints.parameterNames.enabled` | `includeInlayParameterNameHints` |
| `inlayHints.parameterNames.suppressWhenArgumentMatchesName` | `includeInlayParameterNameHintsWhenArgumentMatchesName` (inverted boolean) |
| `inlayHints.parameterTypes.enabled` | `includeInlayFunctionParameterTypeHints` |
| `inlayHints.variableTypes.enabled` | `includeInlayVariableTypeHints` |
| `inlayHints.variableTypes.suppressWhenTypeMatchesName` | `includeInlayVariableTypeHintsWhenTypeMatchesName` (inverted boolean) |
| `inlayHints.propertyDeclarationTypes.enabled` | `includeInlayPropertyDeclarationTypeHints` |
| `inlayHints.functionLikeReturnTypes.enabled` | `includeInlayFunctionLikeReturnTypeHints` |
| `inlayHints.enumMemberValues.enabled` | `includeInlayEnumMemberValueHints` |
| `referencesCodeLens.enabled` | `referencesCodeLensEnabled` |
| `referencesCodeLens.showOnAllFunctions` | `referencesCodeLensShowOnAllFunctions` |
| `implementationsCodeLens.enabled` | `implementationsCodeLensEnabled` |
| `implementationsCodeLens.showOnAllClassMethods` | `implementationsCodeLensShowOnAllClassMethods` |
| `implementationsCodeLens.showOnInterfaceMethods` | `implementationsCodeLensShowOnInterfaceMethods` |
| `format.enabled` | `formatEnabled` |
| `validate.enabled` | `validateEnabled` |
| `workspaceSymbols.excludeLibrarySymbols` | `excludeLibrarySymbolsInNavTo` |
| `tsserver.automaticTypeAcquisition.enabled` | `automaticTypeAcquisitionEnabled` |
| `disableAutomaticTypeAcquisition`, `reportStyleChecksAsWarnings`, `customConfigFileName` | Same name |

TypeScript also accepts `autoClosingTags` as a boolean, `suggest.completeJSDocs`, `format.enable`, and `validate.enable` as compatibility keys when their canonical nested counterparts are absent. `caseSensitivity` also accepts booleans with the same meaning as `organizeImportsIgnoreCase`. `indentStyle` accepts `0`, `1`, and `2` for `none`, `block`, and `smart`.

</details>

[Zed's LSP settings](https://zed.dev/docs/reference/all-settings#lsp) · [TypeScript 7.0.2 startup options](https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/lsp/lsproto/lsp_generated.go#L8774-L8790) · [Workspace preferences](https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/ls/lsutil/userpreferences.go) · [Formatting options](https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/ls/lsutil/formatcodeoptions.go)

### Editor feature presets

The `lsp` block configures the server. Zed's editor settings control which results you see. The example below enables semantic highlighting, LSP folding, inlay hints, and CodeLens. Zed enables linked editing and pull diagnostics by default.

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

[Zed's language-server settings](https://zed.dev/docs/configuring-languages#using-language-server-features) · [Global LSP settings](https://zed.dev/docs/reference/all-settings#global-lsp-settings)

## LSP features unavailable in Zed

TypeScript implements these features. Exposing them requires support in Zed's Rust LSP client. The extension API covers launch, configuration, and labels, with no hook for adding arbitrary LSP requests or editor actions.

| TypeScript LSP feature | Where the limitation is | Why it is unavailable through the fork |
| --- | --- | --- |
| LSP selection ranges (`textDocument/selectionRange`) | Zed core | Zed expands selections using its syntax tree. Its selection action does not request ranges from the server. |
| Range semantic tokens (`textDocument/semanticTokens/range`) | Zed core | Zed requests full-document tokens for semantic highlighting. It does not send the range request. |
| Separate Go to Source Definition action (`custom/textDocument/sourceDefinition`) | Zed core | Zed has no action wired to the custom request. `preferGoToSourceDefinition` lets the existing Go to Definition action prefer source files. |
| Multi-document highlights (`custom/textDocument/multiDocumentHighlight`) | Zed core | Zed requests standard highlights for the current document. Cross-file highlights need client support. |
| Visual Studio grouped references (`textDocument/_vs_references`) | Zed core | Zed uses standard Find References. The grouped response needs a client handler. |
| Visual Studio auto-insert (`textDocument/_vs_onAutoInsert`) | Zed core | Zed does not send this request on trigger characters, so it cannot apply the returned edits. |
| Expandable hovers (`experimental.hoverVerbosityLevel`) | Zed core | Zed shows standard hover text. It does not negotiate TypeScript's verbosity capability or request more detail. |
| Project info and API sessions (`custom/projectInfo`, `custom/initializeAPISession`) | Zed core | Zed has no integration that invokes these TypeScript-specific requests. |
| Content mapper contributions (`custom/setContentMapperContributions`, upstream `main`) | Zed core and TypeScript 7.0.2 | The server handler is present on upstream `main` and absent in 7.0.2. Zed also needs client integration to register and coordinate content mappers. |
| Server telemetry (`telemetry/event`) | Zed core | Zed has no handler for TypeScript server telemetry events. |
| GC, profiling, and live log-level changes (`custom/runGC`, profiling requests, `custom/setLogVerbosity`) | Zed core | Zed has no actions for these developer controls. The `logVerbosity` startup option remains available. |

[TypeScript 7.0.2 server handlers](https://github.com/microsoft/TypeScript/blob/v7.0.2/tsc/internal/lsp/server.go#L745-L815) · [Upstream content mapper handler](https://github.com/microsoft/TypeScript/blob/9c4ad5cbc5a24f3e8893c6e395ac8981c1154c16/tsc/internal/lsp/server.go#L1368) · [Zed extension API](https://docs.rs/zed_extension_api/0.7.0/zed_extension_api/trait.Extension.html)

## Development checks

```sh
cargo clippy --locked --target wasm32-wasip2 -- -D warnings
cargo build --locked --release --target wasm32-wasip2
node --test tests/project_binary.test.cjs
```
