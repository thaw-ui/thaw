use super::{TagPickerInjection, TagPickerSize};
use crate::{TagGroup, TagPickerControlInjection, TagSize};
use leptos::prelude::*;

#[component]
pub fn TagPickerGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let tag_picker = TagPickerInjection::expect_context();
    let TagPickerControlInjection { tag_group_ref, .. } =
        TagPickerControlInjection::expect_context();
    let class = MaybeProp::derive(move || {
        Some(format!(
            "thaw-tag-picker-group {}",
            class.get().unwrap_or_default()
        ))
    });
    let size = Signal::derive(move || match tag_picker.size.get() {
        TagPickerSize::ExtraLarge => TagSize::Medium,
        TagPickerSize::Large => TagSize::Small,
        TagPickerSize::Medium => TagSize::ExtraSmall,
    });
    let on_dismiss = move |value| {
        tag_picker.remove_selected_option(value);
    };

    view! {
        <TagGroup attr:role="listbox" class size dismissible=true on_dismiss node_ref=tag_group_ref>
            {children()}
        </TagGroup>
    }
}
