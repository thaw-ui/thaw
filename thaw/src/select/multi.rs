use leptos::*;
use std::{hash::Hash, rc::Rc, time::Duration};
use thaw_utils::{Model, OptionalProp};

use crate::{
    select::raw::{RawSelect, SelectIcon},
    Icon, SelectLabel, SelectOption, Tag, TagVariant,
};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct MultiSelectOption<T> {
    pub label: String,
    pub value: T,
    pub variant: TagVariant,
}

impl<T> MultiSelectOption<T> {
    pub fn new(label: impl Into<String>, value: T) -> MultiSelectOption<T> {
        MultiSelectOption {
            label: label.into(),
            value,
            variant: TagVariant::Default,
        }
    }

    pub fn with_variant(mut self, variant: TagVariant) -> MultiSelectOption<T> {
        self.variant = variant;
        self
    }
}

impl<T> From<MultiSelectOption<T>> for SelectOption<T> {
    fn from(opt: MultiSelectOption<T>) -> Self {
        SelectOption {
            label: opt.label,
            value: opt.value,
        }
    }
}

#[component]
pub fn MultiSelect<T>(
    #[prop(optional, into)] value: Model<Vec<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<MultiSelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] clearable: MaybeSignal<bool>,
    #[prop(optional)] select_label: Option<SelectLabel>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    let select_options: Signal<Vec<_>> = Signal::derive({
        let options = options.clone();
        move || options.get().into_iter().map(SelectOption::from).collect()
    });
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
    let is_selected = move |v: &T| with!(|value| value.contains(v));
    let on_select: Callback<(ev::MouseEvent, SelectOption<T>)> =
        Callback::new(move |(_, option): (ev::MouseEvent, SelectOption<T>)| {
            let item_value = option.value;
            update!(|value| {
                let index = value
                    .iter()
                    .enumerate()
                    .find_map(|(i, v)| (v == &item_value).then_some(i));
                match index {
                    Some(i) => {
                        value.remove(i);
                    }
                    None => {
                        value.push(item_value);
                    }
                }
            });
        });
    let select_label = select_label.unwrap_or_else(|| {
        let options = options.clone();
        let signal_value = value;
        let value_label = Signal::derive(move || {
            with!(|value, options| {
                value
                    .iter()
                    .map(|value| {
                        let (label, variant) = options
                            .iter()
                            .find(move |v| &v.value == value)
                            .map_or((String::new(), TagVariant::Default), |v| {
                                (v.label.clone(), v.variant)
                            });
                        let value = value.clone();
                        let on_close = Callback::new(move |ev: ev::MouseEvent| {
                            ev.stop_propagation();
                            let value = value.clone();
                            // We remove the item on the next tick to ensure the menu on click handler works correctly
                            set_timeout(
                                move || {
                                    update!(|signal_value| {
                                        let index = signal_value
                                            .iter()
                                            .enumerate()
                                            .find_map(|(i, v)| (v == &value).then_some(i));
                                        if let Some(i) = index {
                                            signal_value.remove(i);
                                        }
                                    });
                                },
                                Duration::ZERO,
                            )
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
            children: Box::new(move || Fragment::new(vec![value_label.into_view()])),
        }
    });
    let is_hovered = RwSignal::new(false);
    let show_clear_icon = Signal::derive(move || {
        clearable.get()
            && ((is_hovered.get() || is_menu_visible.get()) && with!(|value| !value.is_empty()))
    });
    let on_hover_enter = Callback::new(move |_| is_hovered.set(true));
    let on_hover_exit = Callback::new(move |_| is_hovered.set(false));
    let select_icon = SelectIcon {
        children: Rc::new(move || {
            Fragment::new(vec![view! {
                {move || if show_clear_icon.get() {
                    view! {
                        <Icon
                            class="thaw-select-dropdown-icon thaw-select-dropdown-icon--clear"
                            icon=icondata_ai::AiCloseCircleFilled
                            on_click=Callback::new(move |_| {
                                set_timeout(
                                    move || value.set(vec![]),
                                    Duration::ZERO,
                                )
                            })
                        />
                    }
                } else {
                    view! {
                        <Icon class="thaw-select-dropdown-icon" icon=icondata_ai::AiDownOutlined/>
                    }
                }}
            }
            .into_view()])
        }),
    };

    // Trigger the following menu to resync when the value is updated
    let _ = watch(
        move || value.track(),
        move |_, _, _| {
            is_menu_visible.update(|_| {});
        },
        false,
    );

    view! {
        <RawSelect
            options=select_options
            class
            select_label
            select_icon
            is_menu_visible
            on_select=on_select
            show_menu
            hide_menu
            on_hover_enter
            on_hover_exit
            is_selected=is_selected
        />
    }
}
