mod app;
// mod components;
mod pages;

use app::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App)
}
