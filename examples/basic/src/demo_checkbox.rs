use leptos::*;
use melt_ui::*;

#[component]
pub fn DemoCheckout(cx: Scope) -> impl IntoView {
    let (checked, set_checked) = create_signal(cx, false);
    view! {cx,
        <div>
            <Checkbox>
                "A"
            </Checkbox>
            <Checkbox checked=true>
                "B"
            </Checkbox>
            <Checkbox checked=checked on_checked=set_checked>
                "Click"
            </Checkbox>
        </div>
    }
}