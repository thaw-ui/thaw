mod theme;

pub use theme::SelectTheme;

use crate::{theme::use_theme, Tag, TagVariant, Theme};
use leptos::*;
use std::hash::Hash;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[derive(Clone, Default, PartialEq, Eq, Hash)]
pub struct SelectOption<T> {
    pub label: String,
    pub value: T,
    pub variant: TagVariant,
}

impl<T> SelectOption<T> {
    pub fn new(label: impl Into<String>, value: T) -> SelectOption<T> {
        SelectOption {
            label: label.into(),
            value,
            variant: TagVariant::Default,
        }
    }

    pub fn with_variant(mut self, variant: TagVariant) -> SelectOption<T> {
        self.variant = variant;
        self
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum SelectValue<T> {
    Single(Option<T>),
    Multiple(Vec<T>),
}

impl<T> Default for SelectValue<T> {
    fn default() -> Self {
        SelectValue::Single(None)
    }
}

impl From<String> for SelectValue<String> {
    fn from(value: String) -> Self {
        SelectValue::Single(Some(value))
    }
}

impl<T> From<Option<T>> for SelectValue<T> {
    fn from(value: Option<T>) -> Self {
        SelectValue::Single(value)
    }
}

impl<T> From<Vec<T>> for SelectValue<T> {
    fn from(value: Vec<T>) -> Self {
        SelectValue::Multiple(value)
    }
}

#[component]
pub fn Select<T>(
    #[prop(optional, into)] value: Model<SelectValue<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    mount_style("select", include_str!("./select.css"));

    let is_multi = Signal::derive(move || with!(|value| matches!(value, SelectValue::Multiple(_))));

    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.select.border_color
            ));
        });

        css_vars
    });

    let menu_css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.menu_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.select.menu_background_color_hover
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-font-color-selected: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });

    let is_show_menu = create_rw_signal(false);
    let trigger_ref = create_node_ref::<html::Div>();
    let menu_ref = create_node_ref::<html::Div>();
    let show_menu = move |_| {
        is_show_menu.set(true);
    };
    // Trigger the following menu to resync when the value is updated
    let _ = watch(
        move || value.track(),
        move |_, _, _| {
            is_show_menu.update(|_| {});
        },
        false,
    );

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let timer = window_event_listener(ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                if current_el == ***menu_ref.get().unwrap()
                    || current_el == ***trigger_ref.get().unwrap()
                {
                    return;
                }
                el = current_el.parent_element();
            }
            is_show_menu.set(false);
        });
        on_cleanup(move || timer.remove());
    }

    let temp_options = options.clone();
    let select_option_label = create_memo(move |_| {
        let signal_value = value;
        with!(|value, temp_options| match value {
            SelectValue::Single(value) => {
                match value {
                    Some(value) => {
                        let label = temp_options
                            .iter()
                            .find(move |v| &v.value == value)
                            .map_or(String::new(), |v| v.label.clone());
                        label.into_view()
                    }
                    None => view! {}.into_view(),
                }
            }
            SelectValue::Multiple(values) => {
                values
                    .iter()
                    .map(|value| {
                        let (label, variant) = temp_options
                            .iter()
                            .find(move |v| &v.value == value)
                            .map_or((String::new(), TagVariant::Default), |v| {
                                (v.label.clone(), v.variant)
                            });
                        let value = value.clone();
                        let on_close = Callback::new(move |_| {
                            update!(|signal_value| {
                                if let SelectValue::Multiple(values) = signal_value {
                                    let index = values
                                        .iter()
                                        .enumerate()
                                        .find_map(|(i, v)| (v == &value).then_some(i));
                                    if let Some(i) = index {
                                        values.remove(i);
                                    }
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
            }
        })
    });
    let is_selected = move |item_value: &T| {
        with!(|value| match value {
            SelectValue::Single(value) => {
                value.as_ref() == Some(item_value)
            }
            SelectValue::Multiple(values) => {
                values.contains(item_value)
            }
        })
    };
    let select_value = move |item_value: T| {
        update!(|value| match value {
            SelectValue::Single(value) => {
                *value = Some(item_value);
            }
            SelectValue::Multiple(values) => {
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
            }
        });
    };

    view! {
        <Binder target_ref=trigger_ref>
            <div
                class=class_list!["thaw-select", class.map(|c| move || c.get())]
                class=("thaw-select--multiple", is_multi)
                ref=trigger_ref
                on:click=show_menu
                style=move || css_vars.get()
            >
                {move || select_option_label.get()}
            </div>
            <Follower
                slot
                show=is_show_menu
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <CSSTransition
                    node_ref=menu_ref
                    name="fade-in-scale-up-transition"
                    appear=is_show_menu.get_untracked()
                    show=is_show_menu
                    let:display
                >
                    <div
                        class="thaw-select-menu"
                        style=move || {
                            display
                                .get()
                                .map(|d| d.to_string())
                                .unwrap_or_else(|| menu_css_vars.get())
                        }
                        ref=menu_ref
                    >
                        <For
                            each=move || options.get()
                            key=move |item| item.value.clone()
                            children=move |item| {
                                let item = store_value(item);
                                let onclick = move |_| {
                                    let item_value = item.get_value().value;
                                    select_value(item_value);
                                    if !is_multi.get() {
                                        is_show_menu.set(false);
                                    }
                                };
                                view! {
                                    <div
                                        class="thaw-select-menu__item"
                                        class=(
                                            "thaw-select-menu__item-selected",
                                            move || item.with_value(|item_value| is_selected(&item_value.value)),
                                        )

                                        on:click=onclick
                                    >
                                        {item.get_value().label}
                                    </div>
                                }
                            }
                        />
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}
