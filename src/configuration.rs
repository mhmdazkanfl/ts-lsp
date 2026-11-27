use zed_extension_api as z;

pub(super) fn initialization_options() -> z::serde_json::Value {
    z::serde_json::json!({
        "codeLensShowLocationsCommandName": "editor.action.showReferences",
    })
}

pub(super) fn workspace_configuration(
    language_server_id: &z::LanguageServerId,
    worktree: &z::Worktree,
) -> z::Result<z::serde_json::Value> {
    let settings = z::settings::LspSettings::for_worktree(language_server_id.as_ref(), worktree)?;
    let tab_size = z::settings::LanguageSettings::for_worktree(None, worktree)
        .ok()
        .map(|settings| settings.tab_size.get());

    Ok(with_editor_defaults(
        settings.settings,
        tab_size,
    ))
}

fn with_editor_defaults(
    settings: Option<z::serde_json::Value>,
    tab_size: Option<u32>,
) -> z::serde_json::Value {
    let mut settings = settings.unwrap_or_else(|| z::serde_json::json!({}));
    let Some(editor) = settings
        .as_object_mut()
        .and_then(|sections| {
            sections
                .entry("editor")
                .or_insert_with(|| z::serde_json::json!({}))
                .as_object_mut()
        })
    else {
        return settings;
    };

    for (key, value) in [
        (
            "includeInlayParameterNameHints",
            "literals".into(),
        ),
        (
            "includeInlayVariableTypeHints",
            true.into(),
        ),
        ("referencesCodeLensEnabled", true.into()),
        (
            "implementationsCodeLensEnabled",
            true.into(),
        ),
    ] {
        editor.entry(key).or_insert(value);
    }
    if let Some(tab_size) = tab_size {
        for key in ["tabSize", "indentSize"] {
            editor
                .entry(key)
                .or_insert_with(|| tab_size.into());
        }
    }
    settings
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    #[test]
    fn supplies_hint_and_lens_defaults_without_a_zed_tab_width() {
        let settings = super::with_editor_defaults(None, None);

        assert_eq!(
            settings,
            z::serde_json::json!({"editor": {
                "includeInlayParameterNameHints": "literals",
                "includeInlayVariableTypeHints": true,
                "referencesCodeLensEnabled": true,
                "implementationsCodeLensEnabled": true,
            }}),
        );
    }

    #[test]
    fn uses_zed_tab_width_as_the_typescript_indentation_default() {
        let settings = super::with_editor_defaults(None, Some(8));

        assert_eq!(settings["editor"]["tabSize"], 8);
        assert_eq!(settings["editor"]["indentSize"], 8);
    }

    #[test]
    fn fills_missing_editor_defaults_without_overwriting_server_settings() {
        let settings = z::serde_json::json!({
            "editor": {
                "tabSize": 8,
                "convertTabsToSpaces": false,
                "includeInlayParameterNameHints": "none",
                "referencesCodeLensEnabled": false,
            },
            "javascript": {"format": {"semicolons": "remove"}},
            "typescript": {"format": {"indentSize": 4}, "inlayHints": {"variableTypes": {"enabled": false}}},
            "js/ts": {"implementationsCodeLens": {"enabled": false}, "suggest": {"autoImports": false}},
        });

        let configuration = super::with_editor_defaults(Some(settings), Some(2));

        assert_eq!(
            configuration,
            z::serde_json::json!({
                "editor": {
                    "tabSize": 8,
                    "indentSize": 2,
                    "convertTabsToSpaces": false,
                    "includeInlayParameterNameHints": "none",
                    "includeInlayVariableTypeHints": true,
                    "referencesCodeLensEnabled": false,
                    "implementationsCodeLensEnabled": true,
                },
                "javascript": {"format": {"semicolons": "remove"}},
                "typescript": {"format": {"indentSize": 4}, "inlayHints": {"variableTypes": {"enabled": false}}},
                "js/ts": {"implementationsCodeLens": {"enabled": false}, "suggest": {"autoImports": false}},
            }),
        );
    }

    #[test]
    fn preserves_a_non_object_workspace_configuration() {
        let configuration = super::with_editor_defaults(
            Some(z::serde_json::json!(false)),
            Some(2),
        );

        assert_eq!(
            configuration,
            z::serde_json::json!(false)
        );
    }

    #[test]
    fn preserves_an_explicitly_null_editor_configuration() {
        let settings =
            z::serde_json::json!({"editor": null, "typescript": {"format": {"indentSize": 4}}});

        let configuration = super::with_editor_defaults(Some(settings), Some(2));

        assert_eq!(
            configuration,
            z::serde_json::json!({"editor": null, "typescript": {"format": {"indentSize": 4}}}),
        );
    }
}
