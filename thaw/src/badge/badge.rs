use leptos::{either::Either, prelude::*};
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/badge/badge.css");

#[component]
pub fn Badge(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A Badge can be filled, outline, ghost, inverted.
    #[prop(optional, into)]
    appearance: Signal<BadgeAppearance>,
    /// A Badge can be on of several preset sizes.
    #[prop(optional, into)]
    size: Signal<BadgeSize>,
    /// A Badge can be one of preset colors.
    #[prop(optional, into)]
    color: Signal<BadgeColor>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("badge", include_str!("./badge.css"));

    view! {
        <div class=class_list![
            "thaw-badge",
            move || format!("thaw-badge--{}", appearance.get().as_str()),
            move || format!("thaw-badge--{}", size.get().as_str()),
            move || format!("thaw-badge--{}", color.get().as_str()),
            class
        ]>
            {if let Some(children) = children {
                Either::Left(children())
            } else {
                Either::Right(())
            }}
        </div>
    }
}

#[derive(Default, Clone)]
pub enum BadgeAppearance {
    #[default]
    Filled,
    Ghost,
    Outline,
    Tint,
}

impl BadgeAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeAppearance::Filled => "filled",
            BadgeAppearance::Ghost => "ghost",
            BadgeAppearance::Outline => "outline",
            BadgeAppearance::Tint => "tint",
        }
    }
}

#[derive(Default, Clone)]
pub enum BadgeSize {
    Tiny,
    ExtraSmall,
    Small,
    #[default]
    Medium,
    Large,
    ExtraLarge,
}

impl BadgeSize {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeSize::Tiny => "tiny",
            BadgeSize::ExtraSmall => "extra-small",
            BadgeSize::Small => "small",
            BadgeSize::Medium => "medium",
            BadgeSize::Large => "large",
            BadgeSize::ExtraLarge => "extra-large",
        }
    }
}

#[derive(Default, Clone)]
pub enum BadgeColor {
    #[default]
    Brand,
    Danger,
    Important,
    Informative,
    Severe,
    Subtle,
    Success,
    Warning,
}

impl BadgeColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            BadgeColor::Brand => "brand",
            BadgeColor::Danger => "danger",
            BadgeColor::Important => "important",
            BadgeColor::Informative => "informative",
            BadgeColor::Severe => "severe",
            BadgeColor::Subtle => "subtle",
            BadgeColor::Success => "success",
            BadgeColor::Warning => "warning",
        }
    }
}
