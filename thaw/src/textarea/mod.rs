use crate::{FieldInjection, FieldValidationState, Rule};
use leptos::{ev, html, prelude::*};
use std::ops::Deref;
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
    disabled: MaybeSignal<bool>,
    /// Which direction the Textarea is allowed to be resized.
    #[prop(optional, into)]
    resize: MaybeSignal<TextareaResize>,
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

#[derive(Clone)]
pub struct TextareaRef {
    textarea_ref: NodeRef<html::Textarea>,
}

impl TextareaRef {
    /// Focus the input element.
    pub fn focus(&self) {
        if let Some(textarea_el) = self.textarea_ref.get_untracked() {
            _ = textarea_el.focus();
        }
    }

    /// Blur the input element.
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

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TextareaRuleTrigger {
    #[default]
    Blur,
    Focus,
    Input,
    Change,
}

pub struct TextareaRule(Rule<String, TextareaRuleTrigger>);

impl TextareaRule {
    pub fn required(required: MaybeSignal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_empty() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please input!"),
                    |name| format!("Please input {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_empty() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&String, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: TextareaRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for TextareaRule {
    type Target = Rule<String, TextareaRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
