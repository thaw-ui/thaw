use leptos::*;
use thaw_utils::{class_list, mount_style, ComponentRef, Model};

#[component]
pub fn Textarea(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] allow_value: Option<Callback<String, bool>>,
    #[prop(optional, into)] placeholder: MaybeProp<String>,
    #[prop(optional, into)] on_focus: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] on_blur: Option<Callback<ev::FocusEvent>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    /// Which direction the Textarea is allowed to be resized.
    #[prop(optional, into)] resize: MaybeSignal<TextareaResize>,
    #[prop(optional)] comp_ref: ComponentRef<TextareaRef>,
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    mount_style("textarea", include_str!("./textarea.css"));

    let value_trigger = Trigger::new();
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

    let textarea_ref = NodeRef::<html::Textarea>::new();
    textarea_ref.on_load(move |_| {
        comp_ref.load(TextareaRef { textarea_ref });
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
                "thaw-textarea",
                ("thaw-textarea--disabled", move || disabled.get()),
                move || format!("thaw-textarea--resize-{}", resize.get().as_str()),
                class
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
                placeholder=move || placeholder.get()
                ref=textarea_ref
            ></textarea>
        </span>
    }
}

#[derive(Clone)]
pub struct TextareaRef {
    textarea_ref: NodeRef<html::Textarea>,
}

impl TextareaRef {
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

#[derive(Clone, Default)]
pub enum TextareaResize {
    #[default]
    None,
    Both,
    Horizontal,
    Vertical,
}

impl TextareaResize {
    pub fn as_str(&self) -> &'static str {
        match self {
            TextareaResize::None => "none",
            TextareaResize::Both => "both",
            TextareaResize::Horizontal => "horizontal",
            TextareaResize::Vertical => "vertical",
        }
    }
}
