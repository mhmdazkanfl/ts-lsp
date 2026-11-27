# Zed extension manifest

The [JSON Schema](extension.schema.json) describes all 20 top-level keys that Zed recognizes in `extension.toml`, including their nested fields. It is pinned to **Zed 1.31.0 (dev)** at commit [`10113795dffdde78b274c9455fc9c1f3234bf564`][zed-commit]. The editor version and channel come from the pinned source tree. The current manifest format is `schema_version = 1`. [Editor version][zed-version] · [Release channel][zed-channel] · [Manifest parser][manifest]

## Enable autocomplete

This repository's [manifest](../extension.toml) already has a schema directive. To use a local copy of the schema in another project, put this comment at the top of that project's `extension.toml` and adjust the path relative to the manifest.

```toml
#:schema ./docs/extension.schema.json
```

In Zed, install the Tombi extension for TOML completions, hover descriptions, and validation. Tombi and Taplo both recognize the directive. This schema uses JSON Schema Draft 7 with Tombi's `deprecated` and `deprecationMessage` annotations. The descriptions also explain deprecations for editors that display only standard hover text. With the Tombi CLI, run `tombi lint --offline extension.toml` from the repository root to validate against the local schema. [Zed TOML support][zed-toml] · [Tombi schema support][tombi-schema] · [Taplo directives][taplo-directives]

## Extension metadata

| Key | Type | Details |
| --- | --- | --- |
| `id` | String | Required. Unique extension ID. The gallery requires kebab-case and excludes the words `zed` and `extension`. |
| `name` | String | Required. Extension display name. Publishing requires a nonblank name. |
| `version` | String | Required. Extension release version. Use SemVer and match the version in the gallery's `extensions.toml` entry when publishing. |
| `schema_version` | Integer | Required. Use `1` for current manifests. Legacy `0` clears explicit `languages`, `themes`, and `grammars` during discovery. Zed repopulates those fields from disk, using `grammars/*.toml` for grammar declarations. |
| `description` | String | Publishing requires a nonblank description longer than the display name after trimming. |
| `repository` | String | Source repository URL. Publishing requires a valid URL with a host. |
| `authors` | Array of strings | Publishing requires at least one nonblank entry. `"Author Name <author@example.com>"` is a conventional form. |
| `lib.kind` | String | The only value is case-sensitive `"Rust"`. The builder fills this in when it sees `Cargo.toml`. |
| `lib.version` | SemVer string | Zed extension API version. The builder reads it from the compiled WASM's `zed:api-version` section. |

Only `id`, `name`, `version`, and `schema_version` are required by the parser. Fields in `[lib]` are optional. Zed can also load the older `extension.json` format when `extension.toml` is absent. [Manifest and discovery source][manifest] · [WASM version reader][wasm-version] · [Publishing validation][publishing-cli] · [Publishing prerequisites][publishing-policy] · [Publishing guide][publishing-guide]

## Resource paths

All paths in this table are relative to the extension root. The builder adds discovered entries to explicit lists, with the snippet behavior noted below. [Resource discovery][discovery] · [Snippet conventions][snippets]

| Key | Type | Discovery or use |
| --- | --- | --- |
| `languages` | Array of path strings | Language directories. Zed also discovers direct children of `languages/` that contain `config.toml`. |
| `themes` | Array of path strings | Theme JSON files. Zed also discovers JSON files directly under `themes/`. |
| `icon_themes` | Array of path strings | Icon theme JSON files. Zed also discovers JSON files directly under `icon_themes/`. |
| `snippets` | Path string or array of path strings | Snippet JSON files. Zed discovers root `snippets.json` only when the key is omitted. A file named `snippets.json` provides global snippets. Use a lowercase language name for language-specific files. |

```toml
languages = ["languages/my_language"]
snippets = ["snippets/typescript.json", "snippets/snippets.json"]
```

## Tree-sitter grammars

Each `[grammars.<name>]` entry requires a Git `repository` and a revision. Write the revision as `rev` or use the legacy spelling `commit`. Supplying both counts as a duplicate field. The optional `path` is a subdirectory inside the checked-out repository, and Zed looks for `src/parser.c` within it. The grammar name must be in snake_case and match the grammar used in the language's `config.toml`. [Grammar fields][entries] · [Grammar builder][builder] · [Language extensions][language-docs]

## Language servers

Each `[language_servers.<id>]` table uses its key as the ID in Zed settings and in Rust extension API calls. All of its fields are optional to the parser. Provide the Zed languages the server should attach to. [Language server fields][entries] · [Adapter behavior][lsp-adapter]

| Key | Type | Details |
| --- | --- | --- |
| `languages` | Array of strings | Zed language names, with their configured capitalization. |
| `language` | String | Deprecated single-language declaration. Zed uses it only when `languages` is empty or absent. |
| `language_ids` | Table of strings | Maps Zed language names to the LSP `languageId` sent for those languages. Quote TOML keys that contain spaces or dots. |
| `code_action_kinds` | Array of strings | Overrides the adapter's code action kind list. The fallback is `["", "quickfix", "refactor", "refactor.extract", "source"]`. An empty array explicitly replaces the fallback. Server-specific strings are accepted. |
| `opt_in_languages` | Array of strings | Covered languages for which users must list the server's ID explicitly in the `language_servers` setting. The `"..."` selector does not activate it for those languages. The publishing CLI verifies the subset. Older Zed builds without this field start the server according to their usual defaults. |

```toml
[language_servers.my-typescript-server]
languages = ["TypeScript", "TSX"]
language_ids = { TypeScript = "typescript", TSX = "typescriptreact" }
code_action_kinds = ["quickfix", "source.organizeImports"]
opt_in_languages = ["TSX"]
```

Some examples in the pinned language docs show `name` in a server table and use `language-servers` in a dotted header. Zed ignores those unknown keys. Use the underscore spelling `language_servers` and the table key for the server ID. [Pinned language docs][language-docs] · [TOML loader][loader] · [Adapter ID][lsp-adapter]

## MCP servers, debuggers, and legacy entries

| Table | Properties and behavior |
| --- | --- |
| `[context_servers.<id>]` | Empty entry. The extension API supplies the MCP server command and any configuration. Zed is deprecating MCP server extensions in favor of the MCP Registry. New MCP server extension submissions are closed. |
| `[debug_adapters.<id>]` | Optional `schema_path` string, relative to the extension root. Defaults to `debug_adapter_schemas/<id>.json`. The publisher requires the schema file even when the path is omitted. |
| `[debug_locators.<id>]` | Empty entry. The extension implements the named locator in code. |
| `[slash_commands.<id>]` | Required `description` string and `requires_argument` boolean. The parser retains these legacy fields. Slash command extensions are deprecated and the registry no longer accepts submissions. |
| `[language_model_providers.<id>]` | Required `name` display string and optional `icon` SVG path relative to the extension root. The publishing CLI rejects any nonempty map. |

Use headers such as `[context_servers.my-server]` and `[debug_locators.my-locator]` on their own for empty entries. The parser has no `agent_servers` key. Agent servers are distributed through the ACP Registry. [Manifest entries][entries] · [Debugger extensions][debugger-docs] · [Publishing policy][publishing-policy] · [Publishing validation][publishing-cli]

## Capabilities

Use one `[[capabilities]]` table per declaration. The value of `kind` determines which other fields are required. For `args` and `path`, `"*"` matches one element and `"**"` matches all remaining elements, including zero. Use `"**"` as the last element. In `command`, `host`, and `package`, `"*"` matches any value. [Capability kinds][capability-kinds] · [Argument matching][exec-capability] · [URL matching][download-capability] · [Package matching][npm-capability]

| `kind` | Required fields | Host behavior |
| --- | --- | --- |
| `"process:exec"` | `command` string and `args` array of strings. Use `args = []` for zero arguments. | Requires a matching manifest declaration and user grant. |
| `"download_file"` | `host` string and `path` array of URL path-segment strings. | Checks user grants. Uses the URL hostname and path segments for matching. |
| `"npm:install"` | `package` string, including the scope for scoped packages. | Checks user grants. |

```toml
[[capabilities]]
kind = "process:exec"
command = "node"
args = ["--version"]

[[capabilities]]
kind = "download_file"
host = "github.com"
path = ["example", "my-server", "releases", "**"]

[[capabilities]]
kind = "npm:install"
package = "@example/my-language-server"
```

Users control grants through `granted_extension_capabilities` in Zed settings. [Host implementation][granter] · [User settings][capabilities-docs]

## Validation scope

- The schema flags unknown keys to catch typos and ineffective settings. Zed's TOML loader ignores unknown fields, but it rejects malformed values for recognized fields. Keyed tables such as `language_servers` and `language_ids` accept arbitrary IDs and language names. [TOML loader][loader] · [Manifest entries][entries]
- The schema lists the two supported manifest versions. The raw TOML parser reads `schema_version` as a signed 32-bit integer, and Zed checks compatibility when selecting gallery versions. The pinned source contains tests for a proposed schema v2 with per-language settings. Those tests are gated on a future version bump. [Version type][manifest] · [Compatibility check][compatibility] · [Future-version tests][schema-v2-tests]
- Zed's builder and publishing CLI perform filesystem, compatibility, and cross-field checks. These include grammar name casing, path resolution, debug schema validation, publishing metadata, supported feature combinations, and whether the opt-in languages belong to the server. [Builder][builder] · [Publishing CLI][publishing-cli]

[zed-commit]: https://github.com/zed-industries/zed/commit/10113795dffdde78b274c9455fc9c1f3234bf564
[zed-version]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/zed/Cargo.toml#L1-L9
[zed-channel]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/zed/RELEASE_CHANNEL
[manifest]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_manifest.rs
[entries]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_manifest.rs#L213-L320
[loader]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_manifest.rs#L323-L352
[discovery]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_manifest.rs#L353-L483
[schema-v2-tests]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_manifest.rs#L710-L792
[builder]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension_builder.rs
[wasm-version]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/extension.rs#L207-L246
[compatibility]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension_host/src/extension_host.rs#L129-L151
[publishing-cli]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension_cli/src/main.rs
[publishing-policy]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/docs/src/extensions/publishing/prerequisites.md
[publishing-guide]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/docs/src/extensions/publishing/publishing-guide.md
[lsp-adapter]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/language_extension/src/extension_lsp_adapter.rs#L267-L304
[language-docs]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/docs/src/extensions/languages.md
[debugger-docs]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/docs/src/extensions/debugger-extensions.md
[snippets]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/docs/src/extensions/snippets.md
[capability-kinds]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/capabilities.rs
[exec-capability]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/capabilities/process_exec_capability.rs#L3-L40
[download-capability]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/capabilities/download_file_capability.rs#L4-L42
[npm-capability]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension/src/capabilities/npm_install_package_capability.rs#L3-L14
[granter]: https://github.com/zed-industries/zed/blob/10113795dffdde78b274c9455fc9c1f3234bf564/crates/extension_host/src/capability_granter.rs#L23-L83
[capabilities-docs]: https://zed.dev/docs/extensions/capabilities
[zed-toml]: https://zed.dev/docs/languages/toml
[tombi-schema]: https://tombi-toml.github.io/tombi/docs/json-schema/
[taplo-directives]: https://taplo.tamasfe.dev/configuration/directives.html
