use leptos::*;
use thaw_utils::mount_style;

#[component]
pub fn Toaster(toaster_id: String) -> impl IntoView {
    mount_style("toaster", include_str!("./toaster.css"));
    view! {

    }
}
