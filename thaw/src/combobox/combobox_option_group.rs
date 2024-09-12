use super::option_group::OptionGroup;
use leptos::prelude::*;

#[component]
pub fn ComboboxOptionGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Label of the group.
    #[prop(into)]
    label: String,
    children: Children,
) -> impl IntoView {
    view! { <OptionGroup class_prefix="thaw-combobox-option-group" class label children /> }
}
