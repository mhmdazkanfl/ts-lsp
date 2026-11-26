use zed_extension_api as z;

pub fn with_editor_defaults(
    settings: Option<z::serde_json::Value>,
    tab_size: Option<u32>,
) -> Option<z::serde_json::Value> {
    let Some(tab_size) = tab_size else {
        return settings;
    };

    let mut settings = settings.unwrap_or_else(|| z::serde_json::json!({}));
    if let Some(sections) = settings.as_object_mut()
        && let Some(editor) = sections
            .entry("editor")
            .or_insert_with(|| z::serde_json::json!({}))
            .as_object_mut()
    {
        // TypeScript applies its language-specific preferences after the editor defaults.
        editor
            .entry("tabSize")
            .or_insert_with(|| tab_size.into());
        editor
            .entry("indentSize")
            .or_insert_with(|| tab_size.into());
    }
    Some(settings)
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    #[test]
    fn uses_zed_tab_width_as_the_typescript_indentation_default() {
        for tab_size in [2, 8] {
            assert_eq!(
                super::with_editor_defaults(None, Some(tab_size)),
                Some(
                    z::serde_json::json!({"editor": {"tabSize": tab_size, "indentSize": tab_size}})
                ),
                "Zed tab width {tab_size}"
            );
        }
    }

    #[test]
    fn fills_missing_editor_defaults_without_overwriting_server_settings() {
        let settings = z::serde_json::json!({
            "editor": {"tabSize": 8, "convertTabsToSpaces": false},
            "javascript": {"format": {"semicolons": "remove"}},
            "typescript": {"format": {"indentSize": 4}},
            "js/ts": {"suggest": {"autoImports": false}},
        });

        assert_eq!(
            super::with_editor_defaults(Some(settings), Some(2)),
            Some(z::serde_json::json!({
                "editor": {"tabSize": 8, "indentSize": 2, "convertTabsToSpaces": false},
                "javascript": {"format": {"semicolons": "remove"}},
                "typescript": {"format": {"indentSize": 4}},
                "js/ts": {"suggest": {"autoImports": false}},
            })),
        );
    }

    #[test]
    fn preserves_server_settings_when_zed_tab_width_is_unavailable() {
        let settings = Some(z::serde_json::json!({
            "typescript": {"format": {"indentSize": 4}},
        }));

        assert_eq!(
            super::with_editor_defaults(settings.clone(), None),
            settings,
        );
    }
}
