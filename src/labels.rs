mod highlight;

use zed_extension_api as z;

pub(super) fn completion(completion: z::lsp::Completion) -> Option<z::CodeLabel> {
    let highlight = self::highlight::for_completion(completion.kind?)?;

    let label_details = completion.label_details.as_ref();
    let inline_detail = label_details
        .and_then(|details| details.detail.as_deref())
        .unwrap_or_default();
    let description = label_details.and_then(|details| details.description.as_deref());

    Some(completion_label(
        completion.label,
        highlight,
        inline_detail,
        completion.detail.as_deref(),
        description,
    ))
}

fn completion_label(
    name: String,
    highlight: &str,
    inline_detail: &str,
    detail: Option<&str>,
    description: Option<&str>,
) -> z::CodeLabel {
    let secondary = [detail, description]
        .into_iter()
        .flatten()
        .find(|text| !text.is_empty() && *text != name && *text != inline_detail);

    let mut label = highlighted_label(name, highlight);
    if !inline_detail.is_empty() {
        label
            .spans
            .push(z::CodeLabelSpan::literal(
                inline_detail,
                None,
            ));
    }
    if let Some(text) = secondary {
        label
            .spans
            .push(z::CodeLabelSpan::literal(
                format!(" {text}"),
                None,
            ));
    }
    label
}

pub(super) fn symbol(symbol: z::lsp::Symbol) -> Option<z::CodeLabel> {
    let highlight = self::highlight::for_symbol(symbol.kind)?;
    Some(highlighted_label(
        symbol.name,
        highlight,
    ))
}

fn highlighted_label(name: String, highlight: &str) -> z::CodeLabel {
    // Zed's filter range counts bytes in the rendered label and covers only the name.
    let length = name.len();
    z::CodeLabel {
        code: String::new(),
        spans: vec![z::CodeLabelSpan::literal(
            name,
            Some(highlight.into()),
        )],
        filter_range: (0..length).into(),
    }
}

#[cfg(test)]
mod tests {
    use zed_extension_api as z;

    fn assert_label(label: &z::CodeLabel, expected_text: &str, name: &str, highlight: &str) {
        let mut text = String::new();
        let mut highlighted_name = String::new();
        for span in &label.spans {
            match span {
                z::CodeLabelSpan::Literal(span) => {
                    text.push_str(&span.text);
                    if let Some(highlight_name) = &span.highlight_name {
                        assert_eq!(highlight_name, highlight);
                        highlighted_name.push_str(&span.text);
                    }
                }
                z::CodeLabelSpan::CodeRange(range) => {
                    text.push_str(&label.code[range.start as usize..range.end as usize]);
                }
            }
        }

        let filter = label.filter_range.start as usize..label.filter_range.end as usize;
        assert_eq!(text, expected_text);
        assert_eq!(text.get(filter), Some(name));
        assert_eq!(highlighted_name, name);
    }

    #[test]
    fn completion_keeps_the_signature_and_import_source_outside_the_filter_range() {
        let label = super::completion_label(
            "café".into(),
            "function",
            "(size: number)",
            Some("café"),
            Some("./menu"),
        );

        assert_label(
            &label,
            "café(size: number) ./menu",
            "café",
            "function",
        );
    }

    #[test]
    fn completion_uses_resolved_detail_when_available() {
        let label = super::completion(z::lsp::Completion {
            label: "size".into(),
            label_details: None,
            detail: Some("number".into()),
            kind: Some(z::lsp::CompletionKind::Property),
            insert_text_format: None,
        })
        .unwrap();

        assert_label(
            &label,
            "size number",
            "size",
            "property",
        );
    }

    #[test]
    fn workspace_symbol_highlights_the_interface_name() {
        let label = super::symbol(z::lsp::Symbol {
            name: "Menu".into(),
            kind: z::lsp::SymbolKind::Interface,
        })
        .unwrap();

        assert_label(&label, "Menu", "Menu", "type");
    }

    #[test]
    fn workspace_file_symbol_leaves_the_label_to_zed() {
        assert!(
            super::symbol(z::lsp::Symbol {
                name: "menu.ts".into(),
                kind: z::lsp::SymbolKind::File,
            })
            .is_none()
        );
    }
}
