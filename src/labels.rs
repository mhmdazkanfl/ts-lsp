use zed_extension_api::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed_extension_api::{CodeLabel, CodeLabelSpan};

pub(crate) fn completion(completion: Completion) -> Option<CodeLabel> {
    use CompletionKind::*;
    let highlight = match completion.kind? {
        Class | Constructor | Enum | Interface | Struct | TypeParameter => "type",
        Method | Function => "function",
        Property | Field => "property",
        Constant | EnumMember => "constant",
        Variable => "variable",
        Keyword => "keyword",
        _ => return None,
    };
    let mut label = highlighted_label(completion.label, highlight);
    if let Some(detail) = completion.label_details.and_then(|details| details.detail) {
        label.spans.push(CodeLabelSpan::literal(detail, None));
    }
    // Zed renders the description (including the source of auto-imports) separately.
    Some(label)
}

pub(crate) fn symbol(symbol: Symbol) -> Option<CodeLabel> {
    use SymbolKind::*;
    let highlight = match symbol.kind {
        Class | Constructor | Enum | Interface | Struct | TypeParameter => "type",
        Method | Function => "function",
        Property | Field => "property",
        Constant | EnumMember => "constant",
        Variable => "variable",
        _ => return None,
    };
    Some(highlighted_label(symbol.name, highlight))
}

fn highlighted_label(text: String, highlight: &str) -> CodeLabel {
    let len = text.len();
    CodeLabel {
        code: String::new(),
        spans: vec![CodeLabelSpan::literal(text, Some(highlight.to_owned()))],
        filter_range: (0..len).into(),
    }
}
