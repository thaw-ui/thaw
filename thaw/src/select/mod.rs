pub(crate) mod raw;
mod theme;

pub use theme::SelectTheme;

use leptos::*;
use std::{hash::Hash, rc::Rc};
use thaw_utils::{Model, OptionalProp};

use crate::{select::raw::RawSelect, TagVariant};

#[slot]
pub struct SelectLabel {
    children: ChildrenFn,
}

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

#[component]
pub fn Select<T>(
    #[prop(optional, into)] value: Model<Option<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] select_label: Option<SelectLabel>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    let is_menu_visible = create_rw_signal(false);
    let show_menu = move |_| is_menu_visible.set(true);
    let hide_menu = move |_| is_menu_visible.set(false);
    let is_selected = move |v: &T| with!(|value| value.as_ref() == Some(v));
    let on_select: Callback<(ev::MouseEvent, SelectOption<T>)> =
        Callback::new(move |(_, option): (ev::MouseEvent, SelectOption<T>)| {
            let item_value = option.value;
            value.set(Some(item_value));
            hide_menu(());
        });
    let label = select_label.unwrap_or_else(|| {
        let options = options.clone();
        let value_label = Signal::derive(move || {
            with!(|value, options| {
                match value {
                    Some(value) => options
                        .iter()
                        .find(|opt| &opt.value == value)
                        .map_or(String::new(), |v| v.label.clone()),
                    None => String::new(),
                }
            })
        });
        SelectLabel {
            children: Rc::new(move || Fragment::new(vec![value_label.into_view()])),
        }
    });

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
