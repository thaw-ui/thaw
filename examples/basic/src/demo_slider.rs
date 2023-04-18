use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoSlider(cx: Scope) -> impl IntoView {
    view! { cx,
        <Slider value=20.0/>
    }
}
