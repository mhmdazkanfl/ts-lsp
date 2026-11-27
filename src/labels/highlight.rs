use zed_extension_api as z;

enum Highlight {
    Type,
    Constructor,
    Function,
    Method,
    Property,
    Constant,
    Variable,
    Operator,
    Keyword,
}

impl Highlight {
    fn name(self) -> &'static str {
        match self {
            Self::Type => "type",
            Self::Constructor => "constructor",
            Self::Function => "function",
            Self::Method => "function.method",
            Self::Property => "property",
            Self::Constant => "constant",
            Self::Variable => "variable",
            Self::Operator => "operator",
            Self::Keyword => "keyword",
        }
    }
}

pub(super) fn for_completion(kind: z::lsp::CompletionKind) -> Option<&'static str> {
    // Local exception to the full-path style
    // Repeating z::lsp::CompletionKind kinda diabolical
    use z::lsp::CompletionKind::{
        Class, Constant, Constructor, Enum, EnumMember, Field, Function, Interface, Keyword,
        Method, Operator, Property, Struct, TypeParameter, Variable,
    };

    let highlight = match kind {
        Class | Enum | Interface | Struct | TypeParameter => Highlight::Type,
        Constructor => Highlight::Constructor,
        Function => Highlight::Function,
        Method => Highlight::Method,
        Field | Property => Highlight::Property,
        Constant | EnumMember => Highlight::Constant,
        Variable => Highlight::Variable,
        Operator => Highlight::Operator,
        Keyword => Highlight::Keyword,
        _ => return None,
    };
    Some(highlight.name())
}

pub(super) fn for_symbol(kind: z::lsp::SymbolKind) -> Option<&'static str> {
    use z::lsp::SymbolKind::{
        Class, Constant, Constructor, Enum, EnumMember, Field, Function, Interface, Method,
        Operator, Property, Struct, TypeParameter, Variable,
    };

    let highlight = match kind {
        Class | Enum | Interface | Struct | TypeParameter => Highlight::Type,
        Constructor => Highlight::Constructor,
        Function => Highlight::Function,
        Method => Highlight::Method,
        Field | Property => Highlight::Property,
        Constant | EnumMember => Highlight::Constant,
        Variable => Highlight::Variable,
        Operator => Highlight::Operator,
        _ => return None,
    };
    Some(highlight.name())
}
