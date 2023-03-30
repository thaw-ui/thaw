use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Table(cx: Scope, children: Children) -> impl IntoView {
    let class_name = mount_style("table", || style_sheet_str!("./src/table/table.css"));
    view! {cx, class=class_name,
        <table class="melt-table">
            {children(cx)}
        </table>
    }
}
