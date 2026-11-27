use zed_extension_api as z;

pub(super) fn for_completion(kind: z::lsp::CompletionKind) -> Option<&'static str> {
    // Local exception to the full-path style
    // Repeating z::lsp::CompletionKind kinda diabolical
    use z::lsp::CompletionKind::{
        Class, Constant, Constructor, Enum, EnumMember, Field, Function, Interface, Keyword,
        Method, Operator, Property, Struct, TypeParameter, Variable,
    };

    Some(match kind {
        Class | Enum | Interface | Struct | TypeParameter => "type",
        Constructor => "constructor",
        Function => "function",
        Method => "function.method",
        Field | Property => "property",
        Constant | EnumMember => "constant",
        Variable => "variable",
        Operator => "operator",
        Keyword => "keyword",
        _ => return None,
    })
}

pub(super) fn for_symbol(kind: z::lsp::SymbolKind) -> Option<&'static str> {
    use z::lsp::SymbolKind::{
        Class, Constant, Constructor, Enum, EnumMember, Field, Function, Interface, Method,
        Operator, Property, Struct, TypeParameter, Variable,
    };

    Some(match kind {
        Class | Enum | Interface | Struct | TypeParameter => "type",
        Constructor => "constructor",
        Function => "function",
        Method => "function.method",
        Field | Property => "property",
        Constant | EnumMember => "constant",
        Variable => "variable",
        Operator => "operator",
        _ => return None,
    })
}
