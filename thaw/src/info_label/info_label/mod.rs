mod types;

pub use types::*;

use super::info_button::InfoButton;
use crate::Label;
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

/// An InfoLabel is a Label with an InfoButton at the end.
#[component]
pub fn InfoLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A label supports different sizes.
    #[prop(optional, into)]
    size: Signal<InfoLabelSize>,
    /// A label supports regular and semibold fontweight.
    #[prop(optional, into)]
    weight: Signal<InfoLabelWeight>,
    /// Displays an indicator that the label is for a required field.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Renders the label as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    info_label_info: InfoLabelInfo,
    children: Children,
) -> impl IntoView {
    mount_style("info-label", include_str!("./info_label.css"));

    view! {
        <span class=class_list![
            "thaw-info-label",
            move || format!("thaw-info-label--{}", size.get().as_str()),
            class
        ]>
            <Label
                class="thaw-info-label__label"
                required
                disabled
                size=Signal::derive(move || size.get().into())
                weight=Signal::derive(move || weight.get().into())
            >
                {children()}
            </Label>
            <InfoButton
                class="thaw-info-label__info-button"
                size=Signal::derive(move || size.get().into())
            >
                {(info_label_info.children)()}
            </InfoButton>
        </span>
    }
}
