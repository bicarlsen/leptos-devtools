mod component;
mod devtools;
mod extension;
mod runtime;

use extension::on_message;
use leptos_devtools_extension_api::register_leptos;

pub use devtools::Devtools;
pub use runtime::set_cargo_manifest_dir;

pub fn devtools() {
    register_leptos();
    on_message();
}

#[macro_export]
macro_rules! devtools {
    () => {
        leptos_devtools::set_cargo_manifest_dir(env!("CARGO_MANIFEST_DIR").to_string());
        leptos_devtools::devtools();
    };
}
