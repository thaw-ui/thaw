use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Input(
    cx: Scope,
    #[prop(optional, into)] value: MaybeSignal<String>,
    #[prop(optional)] on_input: Option<SignalSetter<String>>,
) -> impl IntoView {
    let class_name = mount_style("modal", || style_sheet_str!("./src/input/input.css"));
    let input_ref = create_node_ref::<html::Input>(cx);
    if let Some(on_input) = on_input {
        input_ref.on_load(cx, move |input| {
            input.on(ev::input, move |ev| {
                on_input.set(event_target_value(&ev));
            });
        });
    }

    view! {
        cx, class=class_name,
        <div class:jo-input=true>
            <input type="text" prop:value=move || value.get() ref=input_ref/>
        </div>
    }
}
