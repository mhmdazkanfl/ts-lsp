use zed_extension_api as zed;

struct TsLspExtension;

impl zed::Extension for TsLspExtension {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(TsLspExtension);
