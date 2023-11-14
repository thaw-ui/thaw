use crate::{theme::use_theme, utils::mount_style, Theme};
use leptos::*;

#[derive(Default, Clone)]
pub enum BadgeVariant {
    Success,
    Warning,
    #[default]
    Error,
}

impl BadgeVariant {
    fn theme_color(&self, theme: &Theme) -> String {
        match self {
            BadgeVariant::Success => theme.common.color_success.clone(),
            BadgeVariant::Warning => theme.common.color_warning.clone(),
            BadgeVariant::Error => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional, into)] value: MaybeSignal<u32>,
    #[prop(default = MaybeSignal::Static(u32::MAX), into)] max: MaybeSignal<u32>,
    #[prop(optional, into)] variant: MaybeSignal<BadgeVariant>,
    #[prop(optional, into)] dot: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("badge", include_str!("./badge.css"));
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        css_vars.push_str("--thaw-font-color: #fff;");
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                variant.get().theme_color(theme)
            ));
        });
        css_vars
    });
    let value = create_memo(move |_| {
        let value = value.get();
        let max_value = max.get();
        if value == 0 {
            String::new()
        } else if max_value < value {
            format!("{max_value}+")
        } else {
            value.to_string()
        }
    });
    view! {
        <div class="thaw-badge" style=move || css_vars.get()>
            <div
                class="thaw-badge__sup"
                class=("thaw-badge__sup--value", move || !dot.get() && !value.get().is_empty())
                class=("thaw-badge__sup--dot", move || dot.get())
            >
                {move || value.get()}
            </div>
            {children()}
        </div>
    }
}
