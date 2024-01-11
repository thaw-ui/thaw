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
    let initial = create_rw_signal(false);
    let item_key = store_value(key);

    create_effect(move |_| {
        initial.set(checkbox_group.0.get().contains(&item_key.get_value()));
    });

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
        <Checkbox class value=checked initial>
            {label}
        </Checkbox>
    }
}
