use leptos::*;
use thaw_utils::{class_list, mount_style, ComponentRef, Model, OptionalProp};

#[component]
pub fn Textarea(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] allow_value: Option<Callback<String, bool>>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional)] comp_ref: ComponentRef<TextAreaRef>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    mount_style("textarea", include_str!("./textarea.css"));

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

    let textarea_ref = create_node_ref::<html::Textarea>();
    textarea_ref.on_load(move |_| {
        comp_ref.load(TextAreaRef { textarea_ref });
    });

    #[cfg(debug_assertions)]
    {
        const INNER_ATTRS: [&str; 3] = ["class", "disabled", "placeholder"];
        attrs.iter().for_each(|attr| {
            if INNER_ATTRS.contains(&attr.0) {
                logging::warn!(
                    "Thaw: The '{}' attribute already exists on elements inside the TextArea component, which may cause conflicts.",
                    attr.0
                );
            }
        });
    }

    view! {
        <span
            class=class_list![
                "thaw-textarea", ("thaw-textarea--focus", move || is_focus.get()),
                ("thaw-textarea--disabled", move || disabled.get()), ("thaw-textarea--invalid", move
                || invalid.get()), class.map(| c | move || c.get())
            ]
        >
            <textarea
                {..attrs}
                prop:value=move || {
                    value_trigger.track();
                    value.get()
                }

                on:input=on_input
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="thaw-textarea__textarea"
                disabled=move || disabled.get()
                placeholder=placeholder.map(|p| move || p.get())
                ref=textarea_ref
            ></textarea>
        </span>
    }
}

#[derive(Clone)]
pub struct TextAreaRef {
    textarea_ref: NodeRef<html::Textarea>,
}

impl TextAreaRef {
    pub fn focus(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.blur();
        }
    }
}
