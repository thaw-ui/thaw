mod app;
mod components;
mod pages;

use app::App;
use leptos::*;

fn main() {
    #[cfg(feature = "tracing")]
    leptos_devtools::devtools!();
    mount_to_body(App)
}
