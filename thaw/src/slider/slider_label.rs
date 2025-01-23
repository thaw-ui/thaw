use crate::SliderInjection;
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn SliderLabel(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Value at which label will be placed.
    #[prop(into)]
    value: Signal<f64>,
    children: Children,
) -> impl IntoView {
    mount_style("slider-label", include_str!("./slider_label.css"));

    let slider = SliderInjection::expect_context();

    let style = move || {
        let value = (value.get() - slider.min.get()) / (slider.max.get() - slider.min.get());

        if slider.vertical.get() {
            format!("bottom: calc({} * (100% - var(--thaw-slider__thumb--size)) + var(--thaw-slider__thumb--size) / 2)", value)
        } else {
            format!("left: calc({} * (100% - var(--thaw-slider__thumb--size)) + var(--thaw-slider__thumb--size) / 2)", value)
        }
    };

    view! {
        <div
            class=class_list![
                "thaw-slider-label",
            move || format!("thaw-slider-label--{}", if slider.vertical.get() { "vertical" } else { "horizontal" }),
            class
            ]
            style=style
        >

            {children()}
        </div>
    }
}
