use crate::{mount_style, theme::use_theme, Theme};
use leptos::*;

#[derive(Default, Clone)]
pub enum BadgeColor {
    Success,
    Warning,
    #[default]
    Error,
}

impl BadgeColor {
    pub fn theme_color(&self, theme: &Theme) -> String {
        match self {
            BadgeColor::Success => theme.common.color_success.clone(),
            BadgeColor::Warning => theme.common.color_warning.clone(),
            BadgeColor::Error => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional, into)] value: MaybeSignal<u32>,
    #[prop(default = MaybeSignal::Static(u32::MAX), into)] max_value: MaybeSignal<u32>,
    #[prop(optional, into)] color: MaybeSignal<BadgeColor>,
    #[prop(optional, into)] dot: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("badge", include_str!("./badge.css"));
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str("--font-color: #fff;");
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--background-color: {};",
                color.get().theme_color(theme)
            ));
        });
        css_vars
    });
    let value = create_memo(move |_| {
        let value = value.get();
        let max_value = max_value.get();
        if value == 0 {
            String::new()
        } else if max_value < value {
            format!("{max_value}+")
        } else {
            value.to_string()
        }
    });
    view! {
        <div class="melt-badge" style=move || css_vars.get()>
            <div
                class="melt-badge__sup"
                class=("melt-badge__sup--value", move || !dot.get() && !value.get().is_empty())
                class=("melt-badge__sup--dot", move || dot.get())
            >
                {move || value.get()}
            </div>
            {children()}
        </div>
    }
}
