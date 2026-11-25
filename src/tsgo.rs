use zed_extension_api as zed;

struct TsgoLs;

impl zed::Extension for TsgoLs {
    fn new() -> Self {
        Self
    }
}

zed::register_extension!(TsgoLs);
