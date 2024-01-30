use crate::{
    checkbox::{checkbox_group::use_checkbox_group, Checkbox},
    utils::OptionalProp,
    SignalWatch,
};
use leptos::*;

#[component]
pub fn CheckboxItem(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] key: String,
) -> impl IntoView {
    let checkbox_group_value = use_checkbox_group().0;
    let checked = create_rw_signal(false);
    let item_key = store_value(key);

    create_effect(move |_| {
        let is_checked =
            checkbox_group_value.with(|value| item_key.with_value(|key| value.contains(key)));

        if is_checked {
            if !checked.get_untracked() {
                checked.set(true);
            }
        } else {
            if checked.get_untracked() {
                checked.set(false);
            }
        }
    });

    _ = checked.watch(move |checked| {
        checkbox_group_value.update(move |value| {
            if *checked {
                value.insert(item_key.get_value());
            } else {
                value.remove(&item_key.get_value());
            }
        });
    });

    let label = if let Some(label) = label {
        label
    } else {
        item_key.get_value()
    };

    view! {
        <Checkbox class value=checked>
            {label}
        </Checkbox>
    }
}
