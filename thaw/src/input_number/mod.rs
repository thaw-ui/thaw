use crate::utils::{Model, OptionalProp, StoredMaybeSignal};
use crate::{Button, ButtonVariant, ComponentRef, Icon, Input, InputRef, InputSuffix};
use leptos::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[component]
pub fn InputNumber<T>(
    #[prop(optional, into)] value: Model<T>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] step: MaybeSignal<T>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] comp_ref: ComponentRef<InputNumberRef>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView
where
    T: Add<Output = T> + Sub<Output = T>,
    T: Default + Clone + FromStr + ToString + 'static,
{
    let input_value = create_rw_signal(String::default());
    create_effect(move |prev| {
        value.with(|value| {
            let value = value.to_string();
            if let Some(prev) = prev {
                if value == prev {
                    return prev;
                }
            }
            input_value.set(value.clone());
            value
        })
    });

    let allow_value = Callback::<String, bool>::new(move |v: String| {
        let Ok(v) = v.parse::<T>() else { return false };
        value.set(v);
        true
    });
    let step: StoredMaybeSignal<_> = step.into();

    let add = Callback::<ev::MouseEvent>::new(move |e: ev::MouseEvent| {
        e.prevent_default();
        value.set(value.get_untracked() + step.get_untracked());
    });
    let sub = Callback::<ev::MouseEvent>::new(move |e: ev::MouseEvent| {
        e.prevent_default();
        value.set(value.get_untracked() - step.get_untracked());
    });

    let input_ref = ComponentRef::<InputRef>::new();
    input_ref.on_load(move |_| {
        comp_ref.load(InputNumberRef { input_ref });
    });

    view! {
        <Input
            attrs
            class
            value=input_value
            allow_value
            placeholder
            disabled
            invalid
            comp_ref=input_ref
        >
            <InputSuffix slot>
                <Button disabled variant=ButtonVariant::Link on_click=sub>
                    <Icon icon=icondata_ai::AiMinusOutlined style="font-size: 18px"/>
                </Button>
                <Button disabled variant=ButtonVariant::Link on_click=add>
                    <Icon icon=icondata_ai::AiPlusOutlined style="font-size: 18px"/>
                </Button>
            </InputSuffix>
        </Input>
    }
}

#[derive(Clone)]
pub struct InputNumberRef {
    input_ref: ComponentRef<InputRef>,
}

impl InputNumberRef {
    pub fn focus(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.blur();
        }
    }
}
