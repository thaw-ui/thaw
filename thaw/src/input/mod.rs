mod text_area;
mod theme;

pub use text_area::{TextArea, TextAreaRef};
pub use theme::InputTheme;

use crate::theme::{use_theme, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, ComponentRef, Model, OptionalProp};

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
pub struct InputPrefix {
    #[prop(default = true)]
    if_: bool,
    children: Children,
}

#[slot]
pub struct InputSuffix {
    #[prop(default = true)]
    if_: bool,
    children: Children,
}

#[component]
pub fn Input(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] allow_value: Option<Callback<String, bool>>,
    #[prop(optional, into)] variant: MaybeSignal<InputVariant>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional)] input_prefix: Option<InputPrefix>,
    #[prop(optional)] input_suffix: Option<InputSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<InputRef>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
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
    let is_focus = create_rw_signal(false);
    let on_internal_focus = move |ev| {
        is_focus.set(true);
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus.call(ev);
        }
    };
    let on_internal_blur = move |ev| {
        is_focus.set(false);
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur.call(ev);
        }
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
            css_vars.push_str(&format!("--thaw-box-shadow-color: {border_color_hover}33;"));
            let border_radius = theme.common.border_radius.clone();
            css_vars.push_str(&format!("--thaw-border-radius: {border_radius};"));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.input.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.input.font_color));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.input.border_color
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color-error: {};",
                theme.common.color_error
            ));
            css_vars.push_str(&format!(
                "--thaw-placeholder-color: {};",
                theme.input.placeholder_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-disabled: {};",
                theme.input.background_color_disabled
            ));
            css_vars.push_str(&format!(
                "--thaw-font-color-disabled: {};",
                theme.input.font_color_disabled
            ));
            css_vars.push_str(&format!(
                "--thaw-box-shadow-color-invalid: {}33;",
                theme.common.color_error
            ));
        });
        css_vars
    });
    let input_ref = create_node_ref::<html::Input>();
    input_ref.on_load(move |_| {
        comp_ref.load(InputRef { input_ref });
    });

    let on_mousedown = move |event: ev::MouseEvent| {
        let el: web_sys::HtmlElement = event_target(&event);

        if el.tag_name() != "INPUT" {
            event.prevent_default();
            if !is_focus.get_untracked() {
                if let Some(comp_ref) = comp_ref.get_untracked() {
                    comp_ref.focus();
                }
            }
        }
    };

    #[cfg(debug_assertions)]
    {
        const INNER_ATTRS: [&str; 4] = ["type", "class", "disabled", "placeholder"];
        attrs.iter().for_each(|attr| {
            if INNER_ATTRS.contains(&attr.0) {
                logging::warn!(
                    "Thaw: The '{}' attribute already exists on elements inside the Input component, which may cause conflicts.",
                    attr.0
                );
            }
        });
    }

    view! {
        <div
            class=class_list![
                "thaw-input", ("thaw-input--focus", move || is_focus.get()),
                ("thaw-input--disabled", move || disabled.get()), ("thaw-input--invalid", move ||
                invalid.get()), class.map(| c | move || c.get())
            ]

            style=move || css_vars.get()
            on:mousedown=on_mousedown
        >
            {if let Some(prefix) = input_prefix.and_then(|prefix| prefix.if_.then_some(prefix)) {
                view! { <div class="thaw-input__prefix">{(prefix.children)()}</div> }.into()
            } else {
                None
            }}

            <input
                {..attrs}
                type=move || variant.get().as_str()
                prop:value=move || {
                    value_trigger.track();
                    value.get()
                }

                on:input=on_input
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="thaw-input__input-el"
                disabled=move || disabled.get()
                placeholder=placeholder.map(|p| move || p.get())
                ref=input_ref
            />

            {if let Some(suffix) = input_suffix.and_then(|suffix| suffix.if_.then_some(suffix)) {
                view! { <div class="thaw-input__suffix">{(suffix.children)()}</div> }.into()
            } else {
                None
            }}

        </div>
    }
}

#[derive(Clone)]
pub struct InputRef {
    input_ref: NodeRef<html::Input>,
}

impl InputRef {
    pub fn focus(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.blur();
        }
    }
}
