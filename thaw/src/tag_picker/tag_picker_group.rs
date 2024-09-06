use crate::{TagGroup, TagSize};
use leptos::prelude::*;
use super::TagPickerInjection;

#[component]
pub fn TagPickerGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let class = MaybeProp::derive(move || {
        Some(format!(
            "thaw-tag-picker-group {}",
            class.get().unwrap_or_default()
        ))
    });
    let on_dismiss = move |value| {
        tag_picker.selected_options.update(|options| {
            if let Some(index) = options.iter().position(|v| v == &value) {
                options.remove(index);
            }
        });
    };

    view! {
        <TagGroup attr:role="listbox" class size=TagSize::ExtraSmall dismissible=true on_dismiss>
            {children()}
        </TagGroup>
    }
}
