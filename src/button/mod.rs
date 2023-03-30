use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Button(cx: Scope, #[prop(default = false)] text: bool, children: Children) -> impl IntoView {
    let class_name = mount_style("button", || style_sheet_str!("./src/button/button.css"));
    let class = move || {
        if text {
            "melt-button melt-button--text"
        } else {
            "melt-button"
        }
    };
    view! {cx, class=class_name,
        <button class=class>
            {children(cx)}
        </button>
    }
}
