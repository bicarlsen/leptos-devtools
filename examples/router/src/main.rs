use leptos::prelude::*;
use leptos_devtools::Devtools;
use router::*;
use tracing_subscriber::{fmt, prelude::*};
use tracing_subscriber_wasm::MakeConsoleWriter;

pub fn main() {
    let fmt_layer = fmt::Layer::default()
        .with_writer(MakeConsoleWriter::default().map_trace_level_to(tracing::Level::DEBUG))
        .without_time()
        .with_ansi(false);
    let devtools_layer = Devtools::default();
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(devtools_layer)
        .init();

    leptos_devtools::devtools!();
    console_error_panic_hook::set_once();
    mount_to_body(RouterExample);
}
