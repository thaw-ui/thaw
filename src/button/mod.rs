use leptos::*;
use stylers::style_sheet;

#[component]
pub fn Button(cx: Scope, #[prop(default = false)] text: bool, children: Children) -> impl IntoView {
    let class_name = style_sheet!("./src/button/button.css");
    let class = move || if text {
        "melt-button melt-button--text"
    } else {
        "melt-button"
    };
    view! {cx, class=class_name,
        <button class=class>
            {children(cx)}
        </button>
    }
}
