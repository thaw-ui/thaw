use crate::checkbox::{checkbox_group::use_checkbox_group, Checkbox};
use leptos::*;
use thaw_utils::OptionalProp;

#[component]
pub fn CheckboxItem(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] key: String,
) -> impl IntoView {
    let checkbox_group_value = use_checkbox_group().0;
    let checked = RwSignal::new(false);
    let item_key = StoredValue::new(key);

    let is_checked = Memo::new(move |_| {
        checkbox_group_value.with(|value| item_key.with_value(|key| value.contains(key)))
    });

    Effect::new(move |prev| {
        checked.track();
        if prev.is_none() {
            return;
        }

        if is_checked.get_untracked() {
            checkbox_group_value.update(move |value| {
                item_key.with_value(|key| {
                    value.remove(key);
                });
            });
        } else if checked.get_untracked() {
            checkbox_group_value.update(move |value| {
                value.insert(item_key.get_value());
            });
        }
    });

    if let Some(label) = label {
        view! {
            <Checkbox class value=(is_checked, checked.write_only())>
                {label}
            </Checkbox>
        }
    } else {
        view! { <Checkbox class value=(is_checked, checked.write_only())/> }
    }
}
