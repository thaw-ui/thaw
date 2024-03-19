use crate::radio::{radio_group::use_radio_group, Radio};
use leptos::*;
use thaw_utils::OptionalProp;

#[component]
pub fn RadioItem(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] key: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let radio_group_value = use_radio_group().0;
    let checked = RwSignal::new(false);
    let item_key = store_value(key);

    let is_checked = Memo::new(move |_| {
        radio_group_value.with(|value| item_key.with_value(|key| value.as_ref() == Some(key)))
    });

    Effect::new(move |prev| {
        let checked = checked.get();
        if prev.is_some() {
            if !is_checked.get_untracked() {
                radio_group_value.set(Some(item_key.get_value()))
            }
        }

        checked
    });

    if let Some(children) = children {
        view! {
            <Radio class value=(is_checked, checked.write_only())>
                {children()}
            </Radio>
        }
    } else {
        view! { <Radio class value=(is_checked, checked.write_only())/> }
    }
}
