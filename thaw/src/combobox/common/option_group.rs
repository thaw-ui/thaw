use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn OptionGroup(
    class_prefix: &'static str,
    class: MaybeProp<String>,
    /// Label of the group.
    label: String,
    children: Children,
) -> impl IntoView {
    mount_style("option-group", include_str!("./option-group.css"));

    view! {
        <div role="group" class=class_list!["thaw-option-group", class_prefix, class]>
            <span
                role="presentation"
                class=format!("thaw-option-group__label {class_prefix}__label")
            >
                {label}
            </span>
            {children()}
        </div>
    }
}
