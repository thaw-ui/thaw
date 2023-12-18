#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::{ssr_class, StoredMaybeSignal};
use crate::{AiIcon, Button, ButtonVariant, Icon, Input, InputSuffix};
use leptos::*;
use std::ops::{Add, Sub};
use std::str::FromStr;

#[component]
pub fn InputNumber<T>(
    #[prop(optional, into)] value: RwSignal<T>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
    #[prop(into)] step: MaybeSignal<T>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: MaybeSignal<String>,
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
    let ssr_class = ssr_class(&class);
    view! {
        <Input
            class=ssr_class
            use:dyn_classes=class
            value=input_value
            allow_value
            placeholder
            disabled
            invalid
        >
            <InputSuffix slot>
                <Button disabled variant=ButtonVariant::Link on_click=sub>
                    <Icon icon=Icon::from(AiIcon::AiMinusOutlined) style="font-size: 18px"/>
                </Button>
                <Button disabled variant=ButtonVariant::Link on_click=add>
                    <Icon icon=Icon::from(AiIcon::AiPlusOutlined) style="font-size: 18px"/>
                </Button>
            </InputSuffix>
        </Input>
    }
}


