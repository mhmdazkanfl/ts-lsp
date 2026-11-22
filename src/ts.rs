mod binary;
mod labels;

use std::cell::OnceCell;

use binary::BinaryResolver;
use zed_extension_api::serde_json::{Value, json};
use zed_extension_api::settings::{CommandSettings, LspSettings};
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

#[derive(Default)]
struct TsGoExtension {
    binaries: BinaryResolver,
}

/// LSP ID previously used before TypeScript 7 was released.
const FALLBACK_KEY: &str = "tsgo";

fn merge_json_value_into(source: Value, target: &mut Value) {
    match (source, target) {
        (Value::Object(source), Value::Object(target)) => {
            for (key, value) in source {
                if let Some(target) = target.get_mut(&key) {
                    merge_json_value_into(value, target);
                } else {
                    target.insert(key, value);
                }
            }
        }

        (Value::Array(source), Value::Array(target)) => target.extend(source),

        (source, target) => *target = source,
    }
}

impl zed::Extension for TsGoExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let settings = lsp_settings_for_worktree(id, worktree)?;
        let options = settings.get_setting(|settings| settings.settings.as_ref());
        let package_version = options
            .and_then(|options| options.get("package_version"))
            .and_then(Value::as_str);
        let prefer_project_binary = options
            .and_then(|options| options.get("prefer_project_binary"))
            .and_then(Value::as_bool)
            .unwrap_or(true);
        self.binaries.command(
            id,
            worktree,
            settings.binary(),
            package_version,
            prefer_project_binary,
        )
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let mut initialization_options = json!({
            "codeLensShowLocationsCommandName": "editor.action.showReferences"
        });

        if let Some(user_initialization_options) = lsp_settings_for_worktree(server_id, worktree)?
            .into_setting(|lsp_settings| lsp_settings.initialization_options)
        {
            merge_json_value_into(user_initialization_options, &mut initialization_options);
        }

        Ok(Some(initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let config = json!({
            "inlayHints": {
                "parameterNames": {
                    "enabled": "all",
                    "suppressWhenArgumentMatchesName": false
                },
                "parameterTypes": {
                    "enabled": true
                },
                "variableTypes": {
                    "enabled": true,
                    "suppressWhenTypeMatchesName": false
                },
                "propertyDeclarationTypes": {
                    "enabled": true
                },
                "functionLikeReturnTypes": {
                    "enabled": true
                },
                "enumMemberValues": {
                    "enabled": true
                }
            },
            "implementationsCodeLens": {
                "enabled": true,
                "showOnAllClassMethods": true,
                "showOnInterfaceMethods": true
            },
            "referencesCodeLens": {
                "enabled": true,
                "showOnAllFunctions": true
            }
        });

        let mut settings = json!({
            "typescript": config.clone(),
            "javascript": config
        });

        if let Some(user_settings) = lsp_settings_for_worktree(server_id, worktree)?
            .into_setting(|lsp_settings| lsp_settings.settings)
        {
            let mut user_settings = user_settings;
            if let Some(object) = user_settings.as_object_mut() {
                object.remove("package_version");
                object.remove("prefer_project_binary");
            }
            merge_json_value_into(user_settings, &mut settings);
        }

        Ok(Some(settings))
    }

    fn label_for_completion(
        &self,
        _: &LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        labels::completion(completion)
    }

    fn label_for_symbol(
        &self,
        _: &LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        labels::symbol(symbol)
    }
}

fn lsp_settings_for_worktree(
    server_id: &LanguageServerId,
    worktree: &Worktree,
) -> Result<LspSettingsWithFallback<impl Fn() -> Option<LspSettings>>> {
    let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)?;
    Ok(LspSettingsWithFallback::new(settings, move || {
        LspSettings::for_worktree(FALLBACK_KEY, worktree).ok()
    }))
}

struct LspSettingsWithFallback<F> {
    settings: LspSettings,
    fallback_settings: OnceCell<Option<LspSettings>>,
    load_fallback: F,
}

impl<F: Fn() -> Option<LspSettings>> LspSettingsWithFallback<F> {
    fn new(settings: LspSettings, load_fallback: F) -> Self {
        Self {
            settings,
            fallback_settings: OnceCell::new(),
            load_fallback,
        }
    }

    fn binary(&self) -> CommandSettings {
        CommandSettings {
            path: self
                .get_setting(|settings| settings.binary.as_ref()?.path.as_ref())
                .cloned(),
            arguments: self
                .get_setting(|settings| settings.binary.as_ref()?.arguments.as_ref())
                .cloned(),
            env: self
                .get_setting(|settings| settings.binary.as_ref()?.env.as_ref())
                .cloned(),
        }
    }

    fn get_setting<R>(&self, f: impl Fn(&LspSettings) -> Option<&R>) -> Option<&R> {
        f(&self.settings).or_else(|| {
            self.fallback_settings
                .get_or_init(&self.load_fallback)
                .as_ref()
                .and_then(f)
        })
    }

    fn into_setting<R>(self, f: impl Fn(LspSettings) -> Option<R>) -> Option<R> {
        f(self.settings).or_else(|| {
            self.fallback_settings
                .into_inner()
                .unwrap_or_else(self.load_fallback)
                .and_then(f)
        })
    }
}

zed::register_extension!(TsGoExtension);

#[cfg(test)]
mod tests {
    use std::cell::Cell;

    use super::*;

    #[test]
    fn fills_missing_binary_fields_from_legacy_settings() {
        let current = zed::serde_json::from_value(json!({
            "binary": { "path": "tools/current-tsc", "arguments": [] }
        }))
        .unwrap();
        let legacy_loads = Cell::new(0);
        let settings = LspSettingsWithFallback::new(current, || {
            legacy_loads.update(|loads| loads + 1);
            Some(
                zed::serde_json::from_value(json!({
                    "binary": {
                        "path": "tools/legacy-tsc",
                        "arguments": ["--legacy"],
                        "env": { "GOMAXPROCS": "4" }
                    },
                    "settings": { "package_version": "7.0.2" }
                }))
                .unwrap(),
            )
        });

        let expected = zed::serde_json::from_value(json!({
            "path": "tools/current-tsc", "arguments": [], "env": { "GOMAXPROCS": "4" }
        }))
        .unwrap();
        assert_eq!(settings.binary(), expected);
        assert_eq!(settings.binary(), expected);
        assert_eq!(
            settings.into_setting(|settings| settings.settings),
            Some(json!({ "package_version": "7.0.2" }))
        );
        assert_eq!(legacy_loads.get(), 1);
    }

    #[test]
    fn prefers_the_current_settings_object_even_when_empty() {
        let current = LspSettings {
            settings: Some(json!({})),
            ..LspSettings::default()
        };
        let settings = LspSettingsWithFallback::new(current, || panic!("unexpected legacy lookup"));

        assert_eq!(
            settings.get_setting(|settings| settings.settings.as_ref()),
            Some(&json!({}))
        );
        assert_eq!(
            settings.into_setting(|settings| settings.settings),
            Some(json!({}))
        );
    }

    #[test]
    fn loads_legacy_initialization_options_when_current_options_are_missing() {
        let settings = LspSettingsWithFallback::new(LspSettings::default(), || {
            Some(LspSettings {
                initialization_options: Some(json!({ "logVerbosity": 2 })),
                ..LspSettings::default()
            })
        });

        assert_eq!(
            settings.into_setting(|settings| settings.initialization_options),
            Some(json!({ "logVerbosity": 2 }))
        );
    }

    #[test]
    fn merges_objects_and_appends_array_items_in_order() {
        let mut target = json!({ "nested": { "keep": true, "list": [1], "replace": false } });
        merge_json_value_into(
            json!({ "nested": { "list": [2, 3], "replace": true } }),
            &mut target,
        );

        assert_eq!(
            target,
            json!({ "nested": { "keep": true, "list": [1, 2, 3], "replace": true } })
        );
    }
}
