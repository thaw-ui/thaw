use crate::{Button, ButtonVariant, ComponentRef, Icon, Input, InputRef, InputSuffix};
use leptos::*;
use num_traits::Bounded;
use std::ops::{Add, Sub};
use std::str::FromStr;
use thaw_utils::{Model, OptionalProp, StoredMaybeSignal};

#[component]
pub fn InputNumber<T>(
    #[prop(optional, into)] value: Model<T>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] step: MaybeSignal<T>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] comp_ref: ComponentRef<InputNumberRef>,
    #[prop(optional, into)] parser: OptionalProp<Callback<String, T>>,
    #[prop(optional, into)] formatter: OptionalProp<Callback<T, String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
    #[prop(default = MaybeSignal::Static(T::min_value()), into)] min: MaybeSignal<T>,
    #[prop(default = MaybeSignal::Static(T::max_value()), into)] max: MaybeSignal<T>,
) -> impl IntoView
where
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Bounded,
    T: Default + Clone + FromStr + ToString + 'static,
{
    let input_value = create_rw_signal(String::default());
    Effect::new_isomorphic(move |prev| {
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
    let min: StoredMaybeSignal<_> = min.into();
    let max: StoredMaybeSignal<_> = max.into();

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

    let set_within_range = Callback::<ev::FocusEvent>::new(move |_| {
        let old_value = value.get_untracked();
        let min = min.get_untracked();
        let max = max.get_untracked();
        if old_value < min {
            value.set(min);
        } else if old_value > max {
            value.set(max);
        }
    });

    let minus_disabled = create_memo(move |_| disabled.get() || value.get() <= min.get());
    let plus_disabled = create_memo(move |_| disabled.get() || value.get() >= max.get());
    let invalid = create_memo(move |_| {
        let value = value.get();
        invalid.get() || value < min.get() || value > max.get()
    });

    let parser = parser.map(|parser| {
        Callback::new(move |v| parser.call(v).to_string())
    });
    let formatter = formatter.map(|formatter| {
        Callback::new(move |v: String| formatter.call(v.parse::<T>().unwrap_or_default()))
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
            on_blur=set_within_range
            parser
            formatter
        >
            <InputSuffix slot>
                <Button disabled=minus_disabled variant=ButtonVariant::Link on_click=sub>
                    <Icon icon=icondata_ai::AiMinusOutlined style="font-size: 18px"/>
                </Button>
                <Button disabled=plus_disabled variant=ButtonVariant::Link on_click=add>
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
