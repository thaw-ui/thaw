use leptos::*;
use thaw_utils::mount_style;

use crate::SliderInjection;

#[component]
pub fn SliderLabel(#[prop(into)] value: MaybeSignal<f64>, children: Children) -> impl IntoView {
    mount_style("slider-label", include_str!("./slider_label.css"));

    let slider = SliderInjection::use_();

    let style = move || {
        let value = (value.get() - slider.min.get()) / (slider.max.get() - slider.min.get());
        format!("left: calc({} * (100% - var(--thaw-slider__thumb--size)) + var(--thaw-slider__thumb--size) / 2)", value)
    };

    view! {
        <option
            class:thaw-slider-label=true
            style=style
            value=move || value.get()
        >

            {children()}
        </option>
    }
}
