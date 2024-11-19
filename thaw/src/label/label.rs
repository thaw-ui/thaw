use leptos::prelude::*;
use thaw_components::{If, Then};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Label(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A label supports different sizes.
    #[prop(optional, into)]
    size: Signal<LabelSize>,
    /// A label supports regular and semibold fontweight.
    #[prop(optional, into)]
    weight: Signal<LabelWeight>,
    /// Displays an indicator that the label is for a required field.
    #[prop(optional, into)]
    required: Signal<bool>,
    /// Renders the label as disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("label", include_str!("./label.css"));

    view! {
        <label class=class_list![
            "thaw-label",
                ("thaw-label--disabled", move || disabled.get()),
                move || format!("thaw-label--{}", size.get().as_str()),
                move || format!("thaw-label--{}", weight.get().as_str()),
                class
        ]>
            {children()} <If cond=required>
                <Then slot>
                    <span aria-hidden="true" class="thaw-label__required">
                        "*"
                    </span>
                </Then>
            </If>
        </label>
    }
}

#[derive(Debug, Default, Clone)]
pub enum LabelSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl LabelSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Small => "small",
            Self::Medium => "medium",
            Self::Large => "large",
        }
    }
}

#[derive(Debug, Default, Clone)]
pub enum LabelWeight {
    #[default]
    Regular,
    Semibold,
}

impl LabelWeight {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Regular => "regular",
            Self::Semibold => "semibold",
        }
    }
}
