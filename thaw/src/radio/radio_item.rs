use crate::{
    radio::{radio_group::use_radio_group, Radio},
    utils::OptionalProp,
    SignalWatch,
};
use leptos::*;

#[component]
pub fn RadioItem(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] key: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let radio_group_value = use_radio_group().0;
    let checked = RwSignal::new(false);
    let item_key = store_value(key);

    create_effect(move |_| {
        let is_checked =
            radio_group_value.with(|value| item_key.with_value(|key| value.as_ref() == Some(key)));

        if is_checked {
            if !checked.get_untracked() {
                checked.set(true);
            }
        } else if checked.get_untracked() {
            checked.set(false);
        }
    });

    _ = checked.watch(move |checked| {
        radio_group_value.update(move |value| {
            if *checked {
                *value = Some(item_key.get_value());
            }
        });
    });

    if let Some(children) = children {
        view! {
            <Radio class value=checked>
                {children()}
            </Radio>
        }
    } else {
        view! {
            <Radio class value=checked>
            </Radio>
        }
    }
}
