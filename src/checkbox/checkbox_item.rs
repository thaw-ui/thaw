use crate::checkbox::{checkbox_group::use_checkbox_group, Checkbox};
use leptos::*;

#[component]
pub fn CheckboxItem(#[prop(into)] label: String, #[prop(into)] value: String) -> impl IntoView {
    let checkbox_group = use_checkbox_group();
    let checked = checkbox_group
        .value
        .with_untracked(|checkbox_group| checkbox_group.contains(&value));
    let checked = create_rw_signal(checked);
    let item_value = store_value(value);

    _ = watch(
        move || checked.get(),
        move |checked, _prev, _| {
            checkbox_group.value.update(move |checkbox_group| {
                if *checked {
                    checkbox_group.insert(item_value.get_value());
                } else {
                    checkbox_group.remove(&item_value.get_value());
                }
            });
        },
        false,
    );

    view! {
        <Checkbox checked>
            { label }
        </Checkbox>
    }
}
