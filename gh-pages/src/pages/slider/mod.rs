use leptos::*;
use melt_ui::*;

#[component]
pub fn SliderPage(cx: Scope) -> impl IntoView {
    let value = create_rw_signal(cx, 0.0);

    view! { cx,
        <Slider value/>
    }
}
