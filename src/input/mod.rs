mod theme;

use crate::{
    theme::{use_theme, Theme},
    utils::{maybe_rw_signal::MaybeRwSignal, mount_style::mount_style},
};
use leptos::*;
pub use theme::InputTheme;

#[derive(Default, Clone)]
pub enum InputVariant {
    #[default]
    Text,
    Password,
}

impl InputVariant {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputVariant::Text => "text",
            InputVariant::Password => "password",
        }
    }
}

#[slot]
pub struct InputSuffix {
    children: Children,
}

#[component]
pub fn Input(
    #[prop(optional, into)] value: MaybeRwSignal<String>,
    #[prop(optional, into)] allow_value: Option<Callback<String, bool>>,
    #[prop(optional, into)] variant: MaybeSignal<InputVariant>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional)] input_suffix: Option<InputSuffix>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    mount_style("input", include_str!("./input.css"));

    let value_trigger = create_trigger();
    let on_input = move |ev| {
        let input_value = event_target_value(&ev);
        if let Some(allow_value) = allow_value.as_ref() {
            if !allow_value.call(input_value.clone()) {
                value_trigger.notify();
                return;
            }
        }
        value.set(input_value);
    };
    let on_internal_focus = move |ev| {
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus.call(ev);
        }
    };
    let on_internal_blur = move |ev| {
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur.call(ev);
        }
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let border_color_hover = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--border-color-hover: {border_color_hover};"));
        let border_radius = theme.common.border_radius.clone();
        css_vars.push_str(&format!("--border-radius: {border_radius};"));
        css_vars
    });
    view! {
        <div class="melt-input" style=move || css_vars.get()>
            <input
                type=move || variant.get().as_str()
                prop:value=move || {
                    value_trigger.track();
                    value.get()
                }
                on:input=on_input
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="melt-input__input-el"
                placeholder=move || placeholder.get()
            />
            {
                if let Some(suffix) = input_suffix {
                    view! {
                        <div class="melt-input__suffix">
                            {(suffix.children)()}
                        </div>
                    }.into()
                } else {
                    None
                }
            }
        </div>
    }
}
