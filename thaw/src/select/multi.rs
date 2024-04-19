use leptos::*;
use std::{hash::Hash, rc::Rc};
use thaw_utils::{Model, OptionalProp};

use crate::{select::raw::RawSelect, SelectLabel, SelectOption, Tag, TagVariant};

#[component]
pub fn MultiSelect<T>(
    #[prop(optional, into)] values: Model<Vec<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] select_label: Option<SelectLabel>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    let class: OptionalProp<_> = match class.into_option() {
        Some(MaybeSignal::Dynamic(class)) => {
            Some(MaybeSignal::Dynamic(Signal::derive(move || {
                with!(|class| format!("thaw-select--multiple {class}"))
            })))
            .into()
        }
        Some(MaybeSignal::Static(class)) => Some(MaybeSignal::Static(format!(
            "thaw-select--multiple {class}"
        )))
        .into(),
        None => Some(MaybeSignal::Static("thaw-select--multiple".to_string())).into(),
    };
    let is_menu_visible = create_rw_signal(false);
    let show_menu = move |_| is_menu_visible.set(true);
    let hide_menu = move |_| is_menu_visible.set(false);
    let is_selected = move |v: &T| with!(|values| values.contains(v));
    let on_select: Callback<(ev::MouseEvent, SelectOption<T>)> =
        Callback::new(move |(_, option): (ev::MouseEvent, SelectOption<T>)| {
            let item_value = option.value;
            update!(|values| {
                let index = values
                    .iter()
                    .enumerate()
                    .find_map(|(i, value)| (value == &item_value).then_some(i));
                match index {
                    Some(i) => {
                        // Deselect
                        values.remove(i);
                    }
                    None => {
                        // Select
                        values.push(item_value);
                    }
                }
            });
        });
    let label = select_label.unwrap_or_else(|| {
        let options = options.clone();
        let signal_values = values;
        let value_label = Signal::derive(move || {
            with!(|values, options| {
                values
                    .iter()
                    .map(|value| {
                        let (label, variant) = options
                            .iter()
                            .find(move |v| &v.value == value)
                            .map_or((String::new(), TagVariant::Default), |v| {
                                (v.label.clone(), v.variant)
                            });
                        let value = value.clone();
                        let on_close = Callback::new(move |_| {
                            update!(|signal_values| {
                                let index = signal_values
                                    .iter()
                                    .enumerate()
                                    .find_map(|(i, v)| (v == &value).then_some(i));
                                if let Some(i) = index {
                                    signal_values.remove(i);
                                }
                            });
                        });
                        view! {
                            <Tag
                                variant
                                closable=true
                                on_close
                            >
                                {label}
                            </Tag>
                        }
                    })
                    .collect_view()
            })
        });
        SelectLabel {
            children: Rc::new(move || Fragment::new(vec![value_label.into_view()])),
        }
    });

    // Trigger the following menu to resync when the value is updated
    let _ = watch(
        move || values.track(),
        move |_, _, _| {
            is_menu_visible.update(|_| {});
        },
        false,
    );

    view! {
        <RawSelect
            options
            class
            label
            is_menu_visible
            on_select=on_select
            show_menu
            hide_menu
            is_selected=is_selected
        />
    }
}
