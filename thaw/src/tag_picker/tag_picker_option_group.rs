use crate::option_group::OptionGroup;
use leptos::prelude::*;

#[component]
pub fn TagPickerOptionGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Label of the group.
    #[prop(into)]
    label: String,
    children: Children,
) -> impl IntoView {
    view! { <OptionGroup class_prefix="thaw-tag-picker-option-group" class label children /> }
}
