use crate::{
    checkbox::{checkbox_group::use_checkbox_group, Checkbox},
    SignalWatch,
};
use leptos::*;

#[component]
pub fn CheckboxItem(
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(into)] key: String,
) -> impl IntoView {
    let checkbox_group = use_checkbox_group();
    let checked = checkbox_group
        .0
        .with_untracked(|checkbox_group| checkbox_group.contains(&key));
    let checked = create_rw_signal(checked);
    let item_key = store_value(key);

    _ = checked.watch(move |checked| {
        checkbox_group.0.update(move |checkbox_group| {
            if *checked {
                checkbox_group.insert(item_key.get_value());
            } else {
                checkbox_group.remove(&item_key.get_value());
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
