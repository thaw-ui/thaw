mod theme;
use crate::{theme::Theme, utils::mount_style::mount_style};
use leptos::*;
use stylers::style_sheet_str;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone)]
pub enum ButtonType {
    #[default]
    PRIMARY,
    SOLID,
    TEXT
}

#[derive(Default)]
pub enum ButtonColor {
    #[default]
    PRIMARY,
    WARNING,
    Error,
}

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional, into)] type_: MaybeSignal<ButtonType>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    children: Children,
) -> impl IntoView {
    // let theme = use_context::<ReadSignal<Theme>>(cx);
    // let css_vars = create_memo(cx, |_| format!("--font-color"));
    let class_name = mount_style("button", || style_sheet_str!("./src/button/button.css"));
    let class = move || {
        if type_.get() == ButtonType::TEXT {
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
