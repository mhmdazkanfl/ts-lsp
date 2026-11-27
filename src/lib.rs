mod configuration;
mod extension;
mod labels;
mod typescript;

use zed_extension_api as z;

z::register_extension!(crate::extension::TsgoLs);
