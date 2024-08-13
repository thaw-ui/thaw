use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Image(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// path to the image you want to display.
    #[prop(optional, into)]
    src: MaybeProp<String>,
    /// description of the image, which isn't mandatory but is incredibly useful for accessibility.
    #[prop(optional, into)]
    alt: MaybeProp<String>,
    /// Image width.
    #[prop(optional, into)]
    width: MaybeProp<String>,
    /// Image height.
    #[prop(optional, into)]
    height: MaybeProp<String>,
    /// An image can appear square, circular, or rounded.
    #[prop(optional, into)]
    shape: MaybeSignal<ImageShape>,
    /// An image can take up the width of its container.
    #[prop(optional, into)]
    block: MaybeSignal<bool>,
    /// An image can appear elevated with shadow.
    #[prop(optional, into)]
    shadow: MaybeSignal<bool>,
    /// An image can set how it should be resized to fit its container.
    #[prop(optional, into)]
    fit: MaybeSignal<ImageFit>,
) -> impl IntoView {
    mount_style("image", include_str!("./image.css"));

    view! {
        <img
            class=class_list![
                "thaw-image",
                ("thaw-image--block", move || block.get()),
                ("thaw-image--shadow", move || shadow.get()),
                move || format!("thaw-image--{}", shape.get().as_str()),
                move || format!("thaw-image--fit-{}", fit.get().as_str()),
                class
            ]
            src=move || src.get()
            alt=move || alt.get()
            width=move || width.get()
            height=move || height.get()
        />
    }
}

#[derive(Default, Clone)]
pub enum ImageShape {
    Circular,
    Rounded,
    #[default]
    Square,
}

impl ImageShape {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageShape::Circular => "circular",
            ImageShape::Rounded => "rounded",
            ImageShape::Square => "square",
        }
    }
}

#[derive(Default, Clone)]
pub enum ImageFit {
    None,
    Contain,
    Cover,
    #[default]
    Fill,
    ScaleDown,
}

impl ImageFit {
    pub fn as_str(&self) -> &'static str {
        match self {
            ImageFit::None => "none",
            ImageFit::Contain => "contain",
            ImageFit::Cover => "cover",
            ImageFit::Fill => "fill",
            ImageFit::ScaleDown => "scale-down",
        }
    }
}
