mod rule;
mod types;

pub use rule::*;
pub use types::*;

use crate::{FieldInjection, Rule};
use leptos::{ev, html, prelude::*};
use thaw_utils::{class_list, mount_style, BoxOneCallback, ComponentRef, Model};

#[component]
pub fn Textarea(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The rules to validate Field.
    #[prop(optional, into)]
    rules: Vec<TextareaRule>,
    /// The value of the Textarea.
    #[prop(optional, into)]
    value: Model<String>,
    /// Check the incoming value, if it returns false, input will not be accepted.
    #[prop(optional, into)]
    allow_value: Option<BoxOneCallback<String, bool>>,
    /// Placeholder text for the input.
    #[prop(optional, into)]
    placeholder: MaybeProp<String>,
    /// Callback triggered when the input is focussed on.
    #[prop(optional, into)]
    on_focus: Option<BoxOneCallback<ev::FocusEvent>>,
    /// Callback triggered when the input is blurred.
    #[prop(optional, into)]
    on_blur: Option<BoxOneCallback<ev::FocusEvent>>,
    /// Whether the input is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Which direction the Textarea is allowed to be resized.
    #[prop(optional, into)]
    resize: Signal<TextareaResize>,
    /// Size of the Textarea.
    #[prop(optional, into)]
    size: Signal<TextareaSize>,
    #[prop(optional)] comp_ref: ComponentRef<TextareaRef>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    mount_style("textarea", include_str!("./textarea.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    let value_trigger = ArcTrigger::new();
    let on_input = {
        let value_trigger = value_trigger.clone();
        move |ev| {
            let input_value = event_target_value(&ev);
            if let Some(allow_value) = allow_value.as_ref() {
                if !allow_value(input_value.clone()) {
                    value_trigger.notify();
                    return;
                }
            }
            value.set(input_value);
            validate.run(Some(TextareaRuleTrigger::Input));
        }
    };
    let on_change = move |_| {
        validate.run(Some(TextareaRuleTrigger::Change));
    };
    let on_internal_focus = move |ev| {
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus(ev);
        }
        validate.run(Some(TextareaRuleTrigger::Focus));
    };
    let on_internal_blur = move |ev| {
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur(ev);
        }
        validate.run(Some(TextareaRuleTrigger::Blur));
    };

    let textarea_ref = NodeRef::<html::Textarea>::new();
    comp_ref.load(TextareaRef { textarea_ref });

    view! {
        <span class=class_list![
            "thaw-textarea",
            ("thaw-textarea--disabled", move || disabled.get()),
            move || format!("thaw-textarea--resize-{}", resize.get().as_str()),
            move || format!("thaw-textarea--{}", size.get().as_str()),
            class
        ]>
            <textarea
                prop:value=move || {
                    value_trigger.track();
                    value.get()
                }

                on:input=on_input
                on:change=on_change
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="thaw-textarea__textarea"
                id=id
                name=name
                disabled=move || disabled.get()
                placeholder=move || placeholder.get()
                node_ref=textarea_ref
            ></textarea>
        </span>
    }
}
