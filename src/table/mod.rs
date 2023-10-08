use crate::utils::mount_style::mount_style;
use leptos::*;

#[component]
pub fn Table(children: Children) -> impl IntoView {
    mount_style("table", include_str!("./table.css"));
    view! { <table class="melt-table">{children()}</table> }
}
