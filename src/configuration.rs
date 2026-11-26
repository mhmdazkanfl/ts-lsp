use zed_extension_api::serde_json::{Value, json};

pub fn with_editor_defaults(settings: Option<Value>, tab_size: Option<u32>) -> Value {
    let mut settings = settings.unwrap_or_else(|| json!({}));
    if let Some(sections) = settings.as_object_mut()
        && let Some(editor) = sections
            .entry("editor")
            .or_insert_with(|| json!({}))
            .as_object_mut()
    {
        // TypeScript applies its language-specific preferences after the editor defaults.
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
            editor
                .entry("tabSize")
                .or_insert_with(|| tab_size.into());
            editor
                .entry("indentSize")
                .or_insert_with(|| tab_size.into());
        }
    }
    settings
}

#[cfg(test)]
mod tests {
    use super::{json, with_editor_defaults};

    #[test]
    fn supplies_hint_and_lens_defaults_without_a_zed_tab_width() {
        assert_eq!(
            with_editor_defaults(None, None),
            json!({"editor": {
                "includeInlayParameterNameHints": "literals",
                "includeInlayVariableTypeHints": true,
                "referencesCodeLensEnabled": true,
                "implementationsCodeLensEnabled": true,
            }}),
        );
    }

    #[test]
    fn uses_zed_tab_width_as_the_typescript_indentation_default() {
        for tab_size in [2, 8] {
            let settings = with_editor_defaults(None, Some(tab_size));
            assert_eq!(settings["editor"]["tabSize"], tab_size);
            assert_eq!(
                settings["editor"]["indentSize"],
                tab_size
            );
        }
    }

    #[test]
    fn fills_missing_editor_defaults_without_overwriting_server_settings() {
        let settings = json!({
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

        assert_eq!(
            with_editor_defaults(Some(settings), Some(2)),
            json!({
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
    fn preserves_non_object_configurations() {
        for settings in [json!(false), json!({"editor": null})] {
            assert_eq!(
                with_editor_defaults(Some(settings.clone()), Some(2)),
                settings,
            );
        }
    }
}
