mod binary;
mod labels;

use binary::BinaryResolver;
use zed_extension_api::serde_json::{Value, json};
use zed_extension_api::settings::{CommandSettings, LspSettings};
use zed_extension_api::{self as zed, LanguageServerId, Result, Worktree};

#[derive(Default)]
struct TsLspExtension {
    binaries: BinaryResolver,
}

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

impl zed::Extension for TsLspExtension {
    fn new() -> Self {
        Self::default()
    }

    fn language_server_command(
        &mut self,
        id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<zed::Command> {
        let settings = LspSettings::for_worktree(id.as_ref(), worktree)?;
        let options = settings.settings.as_ref();
        let package_version = options
            .and_then(|options| options.get("package_version"))
            .and_then(Value::as_str);
        let prefer_project_binary = options
            .and_then(|options| options.get("prefer_project_binary"))
            .and_then(Value::as_bool)
            .unwrap_or(true);
        let binary = settings.binary.unwrap_or(CommandSettings {
            path: None,
            arguments: None,
            env: None,
        });
        self.binaries
            .command(id, worktree, binary, package_version, prefer_project_binary)
    }

    fn language_server_initialization_options(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
        let mut initialization_options = json!({
            "codeLensShowLocationsCommandName": "editor.action.showReferences"
        });

        if let Some(user_initialization_options) =
            LspSettings::for_worktree(server_id.as_ref(), worktree)?.initialization_options
        {
            merge_json_value_into(user_initialization_options, &mut initialization_options);
        }

        Ok(Some(initialization_options))
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<Value>> {
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

        if let Some(mut user_settings) =
            LspSettings::for_worktree(server_id.as_ref(), worktree)?.settings
        {
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

zed::register_extension!(TsLspExtension);

#[cfg(test)]
mod tests {
    use super::*;

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
