use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn ComboboxOptionGroup(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Label of the group.
    #[prop(into)] label: String,
    children: Children,
) -> impl IntoView {
    mount_style(
        "combobox-option-group",
        include_str!("./combobox-option-group.css"),
    );

    view! {
        <div role="group" class=class_list!["thaw-combobox-option-group", class]>
            <span role="presentation" class="thaw-combobox-option-group__label">
                {label}
            </span>
            {children()}
        </div>
    }
}
