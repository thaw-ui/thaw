mod color;
mod theme;

use crate::{mount_style, teleport::Teleport, use_theme, Theme};
pub use color::*;
use leptos::*;
use leptos::{leptos_dom::helpers::WindowListenerHandle, wasm_bindgen::__rt::IntoJsResult};
pub use theme::ColorPickerTheme;

#[component]
pub fn ColorPicker(#[prop(optional, into)] value: RwSignal<RGBA>) -> impl IntoView {
    mount_style("color-picker", include_str!("./color-picker.css"));
    let theme = use_theme(Theme::light);
    let popover_css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--thaw-background-color: {};",
                theme.color_picker.popover_background_color
            )
        })
    });

    let hue = create_rw_signal(0);
    let sv = create_rw_signal((0.0, 0.0));
    let label = create_rw_signal(String::new());
    let style = create_memo(move |_| {
        let mut style = String::new();

        value.with(|value| {
            let value = value.to_hex_string();
            style.push_str(&format!("background-color: {value};"));
            let (s, v) = sv.get_untracked();
            if s < 0.5 && v > 0.5 {
                style.push_str("color: #000;");
            } else {
                style.push_str("color: #fff;");
            }
            label.set(value);
        });

        style
    });

    create_effect(move |prev| {
        let (s, v) = sv.get();
        let hue_value = hue.get();
        if prev.is_none() {
            let HSV {
                hue: h,
                saturation: s,
                value: v,
                ..
            } = value.get_untracked().into();
            hue.set(h);
            sv.set((s, v))
        } else {
            value.set(RGBA::from(HSV::new(hue_value, s, v)));
        }
    });

    let is_show_popover = create_rw_signal(false);
    let trigger_ref = create_node_ref::<html::Div>();
    let popover_ref = create_node_ref::<html::Div>();
    let show_popover = move |_| {
        let rect = trigger_ref.get().unwrap().get_bounding_client_rect();
        is_show_popover.set(true);
        if let Some(popover_ref) = popover_ref.get() {
            _ = popover_ref.style(
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

    view! {
        <div class="thaw-color-picker-trigger" on:click=show_popover ref=trigger_ref>
            <div class="thaw-color-picker-trigger__content" style=move || style.get()>
                {move || label.get()}
            </div>
        </div>
        <Teleport>
            <div
                class="thaw-color-picker-popover"
                ref=popover_ref
                style=move || {
                    if is_show_popover.get() {
                        popover_css_vars.get()
                    } else {
                        "display: none".to_string()
                    }
                }
            >

                <ColorPanel hue=hue.read_only() sv/>
                <HueSlider hue/>
            </div>
        </Teleport>
    }
}

#[component]
fn ColorPanel(hue: ReadSignal<u16>, sv: RwSignal<(f64, f64)>) -> impl IntoView {
    let panel_ref = create_node_ref::<html::Div>();
    let mouse = store_value(Vec::<WindowListenerHandle>::new());

    let on_mouse_down = move |ev| {
        let cb = move |ev: ev::MouseEvent| {
            if let Some(panel) = panel_ref.get_untracked() {
                let rect = panel.get_bounding_client_rect();
                let ev_x = f64::from(ev.x());
                let ev_y = f64::from(ev.y());

                let v = (rect.bottom() - ev_y) / rect.height();
                let s = (ev_x - rect.left()) / rect.width();

                let v = if v > 1.0 {
                    1.0
                } else if v < 0.0 {
                    0.0
                } else {
                    v
                };
                let s = if s > 1.0 {
                    1.0
                } else if s < 0.0 {
                    0.0
                } else {
                    s
                };

                sv.set((s, v))
            }
        };
        cb(ev);
        let on_mouse_move = window_event_listener(ev::mousemove, cb);
        let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
            mouse.update_value(|value| {
                for handle in value.drain(..) {
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
        <div class="thaw-color-picker-popover__panel" ref=panel_ref on:mousedown=on_mouse_down>
            <div
                class="thaw-color-picker-popover__layer"
                style:background-image=move || {
                    format!("linear-gradient(90deg, white, hsl({}, 100%, 50%))", hue.get())
                }
            >
            </div>
            <div class="thaw-color-picker-popover__layer--shadowed"></div>
            <div
                class="thaw-color-picker-popover__handle"
                style=move || {
                    format!(
                        "left: calc({}% - 6px); bottom: calc({}% - 6px)",
                        sv.get().0 * 100.0,
                        sv.get().1 * 100.0,
                    )
                }
            >
            </div>
        </div>
    }
}

#[component]
fn HueSlider(hue: RwSignal<u16>) -> impl IntoView {
    let rail_ref = create_node_ref::<html::Div>();
    let mouse = store_value(Vec::<WindowListenerHandle>::new());

    let on_mouse_down = move |ev| {
        let cb = move |ev: ev::MouseEvent| {
            if let Some(rail) = rail_ref.get_untracked() {
                let rect = rail.get_bounding_client_rect();
                let ev_x = f64::from(ev.x());
                let value = (ev_x - rect.left() - 6.0) / (rect.width() - 12.0) * 359.0;
                let value = if value < 0.0 {
                    0
                } else if value > 359.0 {
                    359
                } else {
                    value.round().to_string().parse::<u16>().unwrap()
                };
                hue.set(value);
            }
        };
        cb(ev);

        let on_mouse_move = window_event_listener(ev::mousemove, cb);
        let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
            mouse.update_value(|value| {
                for handle in value.drain(..) {
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
        <div class="thaw-color-picker-slider" ref=rail_ref on:mousedown=on_mouse_down>
            <div
                class="thaw-color-picker-slider__handle"
                style=move || format!("left: calc({}% - 6px)", f32::from(hue.get()) / 359.0 * 100.0)
            ></div>
        </div>
    }
}
