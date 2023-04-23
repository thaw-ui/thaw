use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoSlider(cx: Scope) -> impl IntoView {
    let (value, set_value) = create_signal(cx, 0.0);
    let on_value = SignalSetter::map(cx, move |value| {
        set_value.set(value);
    });
    view! { cx,
        <Slider value=value on_value=on_value/>
    }
}
