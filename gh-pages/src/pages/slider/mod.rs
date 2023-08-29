use leptos::*;
use melt_ui::*;

#[component]
pub fn SliderPage() -> impl IntoView {
    let value = create_rw_signal(0.0);

    view! {
        <Slider value/>
    }
}
