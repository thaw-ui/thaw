use super::{checkbox_group::CheckboxGroupInjection, Checkbox};
use leptos::*;
use thaw_utils::OptionalProp;

#[component]
pub fn CheckboxItem(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] value: String,
) -> impl IntoView {
    let group_value = CheckboxGroupInjection::use_().0;
    let checked = RwSignal::new(false);
    let item_value = StoredValue::new(value);

    let is_checked = Memo::new(move |_| {
        group_value.with(|group_value| item_value.with_value(|value| group_value.contains(value)))
    });

    Effect::new(move |_| {
        checked.track();

        if is_checked.get_untracked() {
            group_value.update(move |group_value| {
                item_value.with_value(|value| {
                    group_value.remove(value);
                });
            });
        } else if checked.get_untracked() {
            group_value.update(move |group_value| {
                group_value.insert(item_value.get_value());
            });
        }
    });

    if let Some(label) = label {
        view! {
            <Checkbox class checked=(is_checked, checked.write_only())>
                {label}
            </Checkbox>
        }
    } else {
        view! { <Checkbox class checked=(is_checked, checked.write_only())/> }
    }
}
