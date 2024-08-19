use crate::{FieldContextInjection, FieldInjection, FieldValidationState};
use leptos::{ev, html, prelude::*};
use thaw_utils::{
    class_list, mount_style, ArcOneCallback, BoxOneCallback, ComponentRef, Model, OptionalProp,
};

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
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<InputRule>,
    /// Set the input value.
    #[prop(optional, into)]
    value: Model<String>,
    /// Check the incoming value, if it returns false, input will not be accepted.
    #[prop(optional, into)]
    allow_value: Option<ArcOneCallback<String, bool>>,
    /// An input can have different text-based types based on the type of value the user will enter.
    #[prop(optional, into)]
    input_type: MaybeSignal<InputType>,
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
    #[prop(optional)] input_prefix: Option<InputPrefix>,
    #[prop(optional)] input_suffix: Option<InputSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<InputRef>,
    /// Modifies the user input before assigning it to the value.
    #[prop(optional, into)]
    parser: OptionalProp<BoxOneCallback<String, Option<String>>>,
    /// Formats the value to be shown to the user.
    #[prop(optional, into)]
    format: OptionalProp<BoxOneCallback<String, String>>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    mount_style("input", include_str!("./input.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let field_injection = FieldInjection::use_context();
    let rules = StoredValue::new(rules);
    let validate = Callback::new(move |trigger| {
        let state = rules.with_value(move |rules| {
            if rules.is_empty() {
                return None;
            }

            let mut rules_iter = rules.iter();
            loop {
                let Some(rule) = rules_iter.next() else {
                    return None;
                };

                if let Err(state) = rule.call_validator(trigger, value, name) {
                    return Some(state);
                }
            }
        });

        let rt = state.is_none();
        if let Some(field_injection) = field_injection.as_ref() {
            field_injection.update_validation_state(state);
        };
        rt
    });
    if let Some(field_context) = FieldContextInjection::use_context() {
        field_context.register_field(name, move || validate.run(None));
    }

    let parser_none = parser.is_none();
    let on_input = {
        let allow_value = allow_value.clone();
        move |e| {
            if !parser_none {
                validate.run(Some(InputRuleTrigger::Input));
                return;
            }
            let input_value = event_target_value(&e);
            if let Some(allow_value) = allow_value.as_ref() {
                if !allow_value(input_value.clone()) {
                    value.update(|_| {});
                    return;
                }
            }
            value.set(input_value);
            validate.run(Some(InputRuleTrigger::Input));
        }
    };
    let on_change = move |e| {
        let Some(parser) = parser.as_ref() else {
            validate.run(Some(InputRuleTrigger::Change));
            return;
        };
        let Some(parsed_input_value) = parser(event_target_value(&e)) else {
            value.update(|_| {});
            return;
        };
        if let Some(allow_value) = allow_value.as_ref() {
            if !allow_value(parsed_input_value.clone()) {
                value.update(|_| {});
                return;
            }
        }
        value.set(parsed_input_value);
        validate.run(Some(InputRuleTrigger::Change));
    };
    let is_focus = RwSignal::new(false);
    let on_internal_focus = move |ev| {
        is_focus.set(true);
        if let Some(on_focus) = on_focus.as_ref() {
            on_focus(ev);
        }
        validate.run(Some(InputRuleTrigger::Focus));
    };
    let on_internal_blur = move |ev| {
        is_focus.set(false);
        if let Some(on_blur) = on_blur.as_ref() {
            on_blur(ev);
        }
        validate.run(Some(InputRuleTrigger::Blur));
    };

    let input_ref = NodeRef::<html::Input>::new();
    comp_ref.load(InputRef { input_ref });

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

    let input_value: Option<String>;
    #[cfg(feature = "ssr")]
    {
        input_value = Some(value.get_untracked());
    }
    #[cfg(not(feature = "ssr"))]
    {
        input_value = None;
    }

    let prefix_if_ = input_prefix.as_ref().map_or(false, |prefix| prefix.if_);
    let suffix_if_ = input_suffix.as_ref().map_or(false, |suffix| suffix.if_);

    view! {
        <span
            class=class_list![
                "thaw-input",
                ("thaw-input--prefix", prefix_if_),
                ("thaw-input--suffix", suffix_if_),
                ("thaw-input--disabled", move || disabled.get()),
                class
            ]

            on:mousedown=on_mousedown
        >
            {if let Some(prefix) = input_prefix.and_then(|prefix| prefix.if_.then_some(prefix)) {
                view! { <div class="thaw-input__prefix">{(prefix.children)()}</div> }.into()
            } else {
                None
            }}

            <input
                id=id
                type=move || input_type.get().as_str()
                name=name
                value=input_value
                prop:value=move || {
                    let value = value.get();
                    if let Some(format) = format.as_ref() {
                        format(value)
                    } else {
                        value.to_string()
                    }
                }

                on:input=on_input
                on:change=on_change
                on:focus=on_internal_focus
                on:blur=on_internal_blur
                class="thaw-input__input"
                disabled=move || disabled.get()
                placeholder=move || placeholder.get()
                node_ref=input_ref
            />

            {if let Some(suffix) = input_suffix.and_then(|suffix| suffix.if_.then_some(suffix)) {
                view! { <div class="thaw-input__suffix">{(suffix.children)()}</div> }.into()
            } else {
                None
            }}

        </span>
    }
}

#[derive(Default, Clone)]
pub enum InputType {
    #[default]
    Text,
    Password,
    Search,
    Tel,
    Url,
    Email,
    Time,
    Date,
    DatetimeLocal,
    Month,
    Week,
}

impl InputType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Email => "email",
            Self::Time => "time",
            Self::Date => "date",
            Self::DatetimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

#[derive(Clone)]
pub struct InputRef {
    input_ref: NodeRef<html::Input>,
}

impl InputRef {
    /// Focus the input element.
    pub fn focus(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.focus();
        }
    }

    /// Blur the input element.
    pub fn blur(&self) {
        if let Some(input_el) = self.input_ref.get_untracked() {
            _ = input_el.blur();
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum InputRuleTrigger {
    #[default]
    Blur,
    Focus,
    Input,
    Change,
}

enum InputRuleValidator {
    Required(MaybeSignal<bool>),
    RequiredMessage(MaybeSignal<bool>, MaybeSignal<String>),
    Validator(Callback<String, Result<(), FieldValidationState>>),
}

pub struct InputRule {
    validator: InputRuleValidator,
    trigger: InputRuleTrigger,
}

impl InputRule {
    pub fn required(required: MaybeSignal<bool>) -> Self {
        Self {
            trigger: Default::default(),
            validator: InputRuleValidator::Required(required),
        }
    }

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self {
            trigger: Default::default(),
            validator: InputRuleValidator::RequiredMessage(required, message),
        }
    }

    pub fn validator(
        f: impl Fn(&String) -> Result<(), FieldValidationState> + Send + Sync + 'static,
    ) -> Self {
        Self {
            trigger: Default::default(),
            validator: InputRuleValidator::Validator(Callback::from(move |v| f(&v))),
        }
    }

    pub fn with_trigger(mut self, trigger: InputRuleTrigger) -> Self {
        self.trigger = trigger;

        self
    }

    fn call_validator(
        &self,
        trigger: Option<InputRuleTrigger>,
        value: Model<String>,
        name: Signal<Option<String>>,
    ) -> Result<(), FieldValidationState> {
        if let Some(trigger) = trigger {
            if self.trigger != trigger {
                return Ok(());
            }
        }

        value.with_untracked(|value| match &self.validator {
            InputRuleValidator::Required(required) => {
                if required.get_untracked() && value.is_empty() {
                    let message = name.get_untracked().map_or_else(
                        || String::from("Please input!"),
                        |name| format!("Please input {name}!"),
                    );
                    Err(FieldValidationState::Error(message))
                } else {
                    Ok(())
                }
            }
            InputRuleValidator::RequiredMessage(required, message) => {
                if required.get_untracked() && value.is_empty() {
                    Err(FieldValidationState::Error(message.get_untracked()))
                } else {
                    Ok(())
                }
            }
            InputRuleValidator::Validator(f) => f.run(value.clone()),
        })
    }
}
