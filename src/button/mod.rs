mod theme;
use crate::{theme::*, utils::mount_style::mount_style};
use leptos::*;
use leptos_icons::*;
use stylers::style_sheet_str;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonType {
    #[default]
    PRIMARY,
    SOLID,
    TEXT,
    LINK,
}

#[derive(Default, Clone)]
pub enum ButtonColor {
    #[default]
    PRIMARY,
    SUCCESS,
    WARNING,
    ERROR,
}

impl ButtonColor {
    pub fn theme_color(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::PRIMARY => theme.common.color_primary.clone(),
            ButtonColor::SUCCESS => theme.common.color_success.clone(),
            ButtonColor::WARNING => theme.common.color_warning.clone(),
            ButtonColor::ERROR => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Button(
    cx: Scope,
    #[prop(optional, into)] type_: MaybeSignal<ButtonType>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    #[prop(optional, into)] icon: Option<leptos_icons::Icon>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(cx, Theme::light);
    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = color.get().theme_color(&theme);
        if type_.get() == ButtonType::PRIMARY {
            css_vars.push_str(&format!("--background-color: {bg_color};"));
            css_vars.push_str(&format!("--font-color: #fff;"));
            css_vars.push_str(&format!("--border-color: {bg_color};"));
            css_vars.push_str(&format!("--border-color-hover: {bg_color};"));
        } else {
            css_vars.push_str(&format!("--font-color-hover: {bg_color};"));
            css_vars.push_str(&format!("--border-color: #555a;"));
            css_vars.push_str(&format!("--border-color-hover: #555;"));
        }

        css_vars
    });
    let class_name = mount_style("button", || style_sheet_str!("./src/button/button.css"));

    view! {cx, class=class_name,
        <button
            class:melt-button=true
            class=("melt-button--text", move || type_.get() == ButtonType::TEXT)
            class=("melt-button--link", move || type_.get() == ButtonType::LINK)
            style=move || css_vars.get()
            >
            {
                if let Some(icon) = icon {
                    view!{cx,
                            <LeptosIcon icon=icon style="margin-right: 6px"/>
                    }.into()
                } else {
                    None
                }
            }
            {children(cx)}
        </button>
    }
}
