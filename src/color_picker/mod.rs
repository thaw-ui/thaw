use crate::{mount_style, teleport::Teleport, utils::maybe_rw_signal::MaybeRwSignal};
use leptos::*;
use leptos_dom::helpers::WindowListenerHandle;
use wasm_bindgen::__rt::IntoJsResult;

#[component]
pub fn ColorPicker(#[prop(optional, into)] value: MaybeRwSignal<String>) -> impl IntoView {
    mount_style("color-picker", include_str!("./color-picker.css"));
    let label = create_rw_signal(String::new());
    let style = create_memo(move |_| {
        let mut style = String::new();

        value.with(|value| {
            if value.is_empty() {
                label.set("Invalid value".into());
                return;
            }

            style.push_str(&format!("background-color: {value}"));
            label.set(value.clone());
        });

        style
    });

    let is_show_popover = create_rw_signal(false);
    let trigger_ref = create_node_ref::<html::Div>();
    let popover_ref = create_node_ref::<html::Div>();
    let show_popover = move |_| {
        let rect = trigger_ref.get().unwrap().get_bounding_client_rect();
        is_show_popover.set(true);
        if let Some(popover_ref) = popover_ref.get() {
            popover_ref.style(
                "transform",
                format!(
                    "translateX({}px) translateY({}px)",
                    rect.x(),
                    rect.y() + rect.height()
                ),
            );
        }
    };
    let timer = window_event_listener(ev::click, move |ev| {
        let el = ev.target();
        let mut el: Option<web_sys::Element> =
            el.into_js_result().map_or(None, |el| Some(el.into()));
        let body = document().body().unwrap();
        while let Some(current_el) = el {
            if current_el == *body {
                break;
            };
            if current_el == ***popover_ref.get().unwrap()
                || current_el == ***trigger_ref.get().unwrap()
            {
                return;
            }
            el = current_el.parent_element();
        }
        is_show_popover.set(false);
    });
    on_cleanup(move || timer.remove());
    let hue = create_rw_signal(0);
    view! {
        <div class="melt-color-picker-trigger" on:click=show_popover ref=trigger_ref>
            <div class="melt-color-picker-trigger__content" style=move || style.get()>
                {move || label.get()}
            </div>
        </div>
        <Teleport>
            <div
                class="melt-color-picker-popover"
                ref=popover_ref
                style=move || {
                    if !is_show_popover.get() { format!("display: none").into() } else { None }
                }
            >

                <div class="melt-color-picker-popover__panel">
                    <div class="melt-color-picker-popover__layer"></div>
                    <div class="melt-color-picker-popover__layer--shadowed"></div>
                    <div></div>
                </div>
                <HueSlider hue/>
            </div>
        </Teleport>
    }
}

#[component]
fn HueSlider(hue: RwSignal<u16>) -> impl IntoView {
    let rail_ref = create_node_ref::<html::Div>();
    let mouse = store_value(Vec::<WindowListenerHandle>::new());

    let on_mouse_down = move |_| {
        let on_mouse_move = window_event_listener(ev::mousemove, move |ev| {
            if let Some(rail) = rail_ref.get_untracked() {
                let rect = rail.get_bounding_client_rect();
                let ev_x = f64::from(ev.x());
                let value = (ev_x - rect.x()) / (rect.width() - 12.0) * 360.0;
                let value = if value < 0.0 {
                    0
                } else if value > 359.0 {
                    359
                } else {
                    value.round().to_string().parse::<u16>().unwrap()
                };
                hue.set(value);
            }
        });
        let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
            mouse.update_value(|value| {
                for handle in value.drain(..).into_iter() {
                    handle.remove();
                }
            });
        });
        mouse.update_value(|value| {
            value.push(on_mouse_move);
            value.push(on_mouse_up);
        });
    };
    view! {
        <div class="melt-color-picker-slider" ref=rail_ref>
            <div
                class="melt-color-picker-slider__handle"
                on:mousedown=on_mouse_down
                style=move || format!("left: {}%", f32::from(hue.get()) / 359.0 * 100.0)
            ></div>
        </div>
    }
}
