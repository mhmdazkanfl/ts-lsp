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

        (Value::Array(source), Value::Array(target)) => {
            for value in source {
                target.push(value);
            }
        }

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
        let settings = LspSettingsWithFallback::for_worktree(id, FALLBACK_KEY, worktree)?;
        let options = settings.get_setting(|settings| settings.settings.as_ref());
        let package_version = options
            .and_then(|options| options.get("package_version"))
            .and_then(Value::as_str);
        let prefer_project_binary = options
            .and_then(|options| options.get("prefer_project_binary"))
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let binary = CommandSettings {
            path: settings
                .get_setting(|settings| settings.binary.as_ref()?.path.as_ref())
                .cloned(),
            arguments: settings
                .get_setting(|settings| settings.binary.as_ref()?.arguments.as_ref())
                .cloned(),
            env: settings
                .get_setting(|settings| settings.binary.as_ref()?.env.as_ref())
                .cloned(),
        };
        self.binaries
            .command(id, worktree, binary, package_version, prefer_project_binary)
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &zed_extension_api::LanguageServerId,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<zed_extension_api::serde_json::Value>> {
        let mut initialization_options = json!({
            "codeLensShowLocationsCommandName": "editor.action.showReferences"
        });

        if let Some(user_initialization_options) =
            LspSettingsWithFallback::for_worktree(server_id, FALLBACK_KEY, worktree)?
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

        if let Some(user_settings) =
            LspSettingsWithFallback::for_worktree(server_id, FALLBACK_KEY, worktree)?
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

struct LspSettingsWithFallback<'a> {
    settings: LspSettings,
    fallback_id: &'static str,
    fallback_settings: OnceCell<Option<LspSettings>>,
    worktree: &'a Worktree,
}

impl<'a> LspSettingsWithFallback<'a> {
    fn for_worktree(
        server_id: &LanguageServerId,
        fallback_id: &'static str,
        worktree: &'a Worktree,
    ) -> zed_extension_api::Result<Self> {
        LspSettings::for_worktree(server_id.as_ref(), worktree).map(|settings| Self {
            settings,
            fallback_id,
            fallback_settings: OnceCell::new(),
            worktree,
        })
    }

    fn get_setting<F, R>(&self, f: F) -> Option<&R>
    where
        F: Fn(&LspSettings) -> Option<&R>,
    {
        f(&self.settings).or_else(|| {
            self.fallback_settings
                .get_or_init(|| LspSettings::for_worktree(self.fallback_id, self.worktree).ok())
                .as_ref()
                .and_then(f)
        })
    }

    fn into_setting<F, R>(self, f: F) -> Option<R>
    where
        F: Fn(LspSettings) -> Option<R>,
    {
        f(self.settings).or_else(|| {
            let _ = self
                .fallback_settings
                .get_or_init(|| LspSettings::for_worktree(self.fallback_id, self.worktree).ok());
            self.fallback_settings.into_inner().flatten().and_then(f)
        })
    }
}

zed::register_extension!(TsGoExtension);
