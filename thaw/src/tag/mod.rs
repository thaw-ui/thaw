mod theme;

pub use theme::TagTheme;

use crate::{theme::use_theme, Icon, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum TagVariant {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

impl TagVariant {
    fn theme_font_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_font_color.clone(),
            TagVariant::Success => theme.common.color_success.clone(),
            TagVariant::Warning => theme.common.color_warning.clone(),
            TagVariant::Error => theme.common.color_error.clone(),
        }
    }
    fn theme_background_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_background_color.clone(),
            TagVariant::Success => theme.tag.success_background_color.clone(),
            TagVariant::Warning => theme.tag.warning_background_color.clone(),
            TagVariant::Error => theme.tag.error_background_color.clone(),
        }
    }
    fn theme_border_color(&self, theme: &Theme) -> String {
        match self {
            TagVariant::Default => theme.tag.default_border_color.clone(),
            TagVariant::Success => theme.tag.success_border_color.clone(),
            TagVariant::Warning => theme.tag.warning_border_color.clone(),
            TagVariant::Error => theme.tag.error_border_color.clone(),
        }
    }
}

#[component]
pub fn Tag(
    #[prop(optional, into)] variant: MaybeSignal<TagVariant>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] closable: MaybeSignal<bool>,
    #[prop(optional, into)] on_close: Option<Callback<ev::MouseEvent>>,
    children: Children,
) -> impl IntoView {
    mount_style("tag", include_str!("./tag.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let variant = variant.get();
            css_vars.push_str(&format!(
                "--thaw-font-color: {};",
                variant.theme_font_color(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                variant.theme_background_color(theme)
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                variant.theme_border_color(theme)
            ));
        });
        css_vars
    });

    let on_close = move |event| {
        let Some(callback) = on_close.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {
        <div
            class=class_list!["thaw-tag", class.map(| c | move || c.get())]
            style=move || css_vars.get()
        >
            <span class="thaw-tag__content">{children()}</span>

            {move || {
                if closable.get() {
                    view! {
                        <button class="thaw-tag__close" on:click=on_close>
                            <Icon icon=icondata_ai::AiCloseOutlined style="font-size: 14px"/>
                        </button>
                    }
                        .into()
                } else {
                    None
                }
            }}

        </div>
    }
}
