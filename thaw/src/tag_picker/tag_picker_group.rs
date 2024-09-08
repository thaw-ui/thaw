use super::TagPickerInjection;
use crate::{TagGroup, TagSize};
use leptos::prelude::*;

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
        tag_picker.remove_selected_option(value);
    };

    view! {
        <TagGroup attr:role="listbox" class size=TagSize::ExtraSmall dismissible=true on_dismiss>
            {children()}
        </TagGroup>
    }
}
