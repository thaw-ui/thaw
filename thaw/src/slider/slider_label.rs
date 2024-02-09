use crate::utils::mount_style;
use leptos::*;

#[component]
pub fn SliderLabel(#[prop(into)] value: MaybeSignal<f64>, children: Children) -> impl IntoView {
    mount_style("slider-label", include_str!("./slider_label.css"));

    view! {
        <div
            class:thaw-slider-label=true
            style=move || {
                format!("left: calc(calc({} / var(--thaw-slider-max)) * 100%)", value.get())
            }
        >

            {children()}
        </div>
    }
}
