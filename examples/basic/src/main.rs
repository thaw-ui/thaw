mod app;
mod demo_button;
mod demo_checkbox;
mod demo_modal;
mod demo_slider;
mod pages;

use app::*;
use leptos::*;

fn main() {
    mount_to_body(|cx| view! { cx, <App/> })
}
