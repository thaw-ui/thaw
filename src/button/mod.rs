mod theme;

use crate::{
    components::{OptionComp, Wave, WaveRef},
    icon::*,
    theme::*,
    utils::{mount_style, ComponentRef},
};
use leptos::*;
pub use theme::ButtonTheme;

#[derive(Default, PartialEq, Clone, Copy)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Solid,
    Text,
    Link,
}

#[derive(Default, Clone)]
pub enum ButtonColor {
    #[default]
    Primary,
    Success,
    Warning,
    Error,
}

impl ButtonColor {
    fn theme_color(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary.clone(),
            ButtonColor::Success => theme.common.color_success.clone(),
            ButtonColor::Warning => theme.common.color_warning.clone(),
            ButtonColor::Error => theme.common.color_error.clone(),
        }
    }
    fn theme_color_hover(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary_hover.clone(),
            ButtonColor::Success => theme.common.color_success_hover.clone(),
            ButtonColor::Warning => theme.common.color_warning_hover.clone(),
            ButtonColor::Error => theme.common.color_error_hover.clone(),
        }
    }

    fn theme_color_active(&self, theme: &Theme) -> String {
        match self {
            ButtonColor::Primary => theme.common.color_primary_active.clone(),
            ButtonColor::Success => theme.common.color_success_active.clone(),
            ButtonColor::Warning => theme.common.color_warning_active.clone(),
            ButtonColor::Error => theme.common.color_error_active.clone(),
        }
    }
}

#[component]
pub fn Button(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(optional, into)] variant: MaybeSignal<ButtonVariant>,
    #[prop(optional, into)] color: MaybeSignal<ButtonColor>,
    #[prop(optional, into)] round: MaybeSignal<bool>,
    #[prop(optional, into)] icon: Option<Icon>,
    #[prop(optional, into)] loading: MaybeSignal<bool>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let bg_color = color.get().theme_color(theme);
            if variant.get() == ButtonVariant::Primary {
                let bg_color_hover = color.get().theme_color_hover(theme);
                let bg_color_active = color.get().theme_color_active(theme);
                css_vars.push_str(&format!("--thaw-background-color: {bg_color};"));
                css_vars.push_str(&format!("--thaw-background-color-hover: {bg_color_hover};"));
                css_vars.push_str(&format!(
                    "--thaw-background-color-active: {bg_color_active};"
                ));
                css_vars.push_str("--thaw-font-color: #fff;");
                css_vars.push_str(&format!("--thaw-border-color: {bg_color};"));
                css_vars.push_str(&format!("--thaw-border-color-hover: {bg_color};"));
                css_vars.push_str(&format!("--thaw-ripple-color: {bg_color};"));
            } else if variant.get() == ButtonVariant::Text {
                css_vars.push_str(&format!("--thaw-font-color-hover: {bg_color};"));
                css_vars.push_str(&format!(
                    "--thaw-background-color-hover: {};",
                    theme.button.color_text_hover
                ));
                css_vars.push_str(&format!(
                    "--thaw-background-color-active: {};",
                    theme.button.color_text_active
                ));
                css_vars.push_str("--thaw-ripple-color: #0000;");
            } else {
                css_vars.push_str(&format!("--thaw-font-color-hover: {bg_color};"));
                css_vars.push_str("--thaw-border-color: #555a;");
                css_vars.push_str("--thaw-border-color-hover: #555;");
                css_vars.push_str("--thaw-ripple-color: #0000;");
            }
        });

        css_vars
    });
    mount_style("button", include_str!("./button.css"));

    let icon_style = if children.is_some() {
        "margin-right: 6px"
    } else {
        ""
    };

    let disabled = create_memo(move |_| {
        if loading.get() {
            return true;
        }

        disabled.get()
    });

    let wave_ref = ComponentRef::<WaveRef>::default();

    let on_click = move |event| {
        if disabled.get() {
            return;
        }
        if let Some(wave_ref) = wave_ref.get_untracked() {
            wave_ref.play();
        }
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {
        <button
            class:thaw-button=true
            class=("thaw-button--text", move || variant.get() == ButtonVariant::Text)
            class=("thaw-button--link", move || variant.get() == ButtonVariant::Link)
            class=("thaw-button--round", move || round.get())
            class=("thaw-button--disabled", move || disabled.get())
            style=move || format!("{}{}", css_vars.get(), style.get())
            disabled=move || disabled.get()
            on:click=on_click
        >
            <Wave comp_ref=wave_ref/>
            {move || {
                if loading.get() {
                    view! {
                        <Icon
                            icon=Icon::from(AiIcon::AiLoadingOutlined)
                            style=format!(
                                "animation: thawLoadingCircle 1s infinite linear;{icon_style}",
                            )
                        />
                    }
                        .into()
                } else if let Some(icon) = icon {
                    view! { <Icon icon=icon style=icon_style/> }.into()
                } else {
                    None
                }
            }}

            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </button>
    }
}
