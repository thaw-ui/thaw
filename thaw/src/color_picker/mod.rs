mod color;

pub use color::*;

use crate::ConfigInjection;
use leptos::leptos_dom::helpers::WindowListenerHandle;
use leptos::{ev, html, prelude::*};
use palette::{Hsv, IntoColor, Srgb};
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn ColorPicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Value of the picker.
    #[prop(optional, into)]
    value: Model<Color>,
) -> impl IntoView {
    mount_style("color-picker", include_str!("./color-picker.css"));
    let config_provider = ConfigInjection::expect_context();
    let hue = RwSignal::new(0f32);
    let sv = RwSignal::new((0f32, 0f32));
    let label = RwSignal::new(String::new());
    let style = Memo::new(move |_| {
        let mut style = String::new();

        value.with(|color| {
            let (s, v) = sv.get_untracked();
            if s < 0.5 && v > 0.5 {
                style.push_str("color: #000;");
            } else {
                style.push_str("color: #fff;");
            }
            match color {
                Color::RGB(rgb) => {
                    let rgb = Srgb::<u8>::from_format(rgb.clone());
                    let color = format!("rgb({}, {}, {})", rgb.red, rgb.green, rgb.blue);
                    style.push_str(&format!("background-color: {color};"));
                    label.set(color);
                }
                Color::HSV(hsv) => {
                    let rgb: Srgb = hsv.clone().into_color();
                    let rgb = Srgb::<u8>::from_format(rgb);
                    let color = format!("rgb({}, {}, {})", rgb.red, rgb.green, rgb.blue);
                    style.push_str(&format!("background-color: {color};"));

                    let color = format!(
                        "hsv({}, {:.0}%, {:.0}%)",
                        hsv.hue.into_inner(),
                        hsv.saturation * 100.0,
                        hsv.value * 100.0
                    );
                    label.set(color);
                }
                Color::HSL(hsl) => {
                    let color = format!(
                        "hsl({}, {:.0}%, {:.0}%)",
                        hsl.hue.into_inner(),
                        hsl.saturation * 100.0,
                        hsl.lightness * 100.0
                    );
                    style.push_str(&format!("background-color: {color};"));
                    label.set(color);
                }
            }
        });

        style
    });

    Effect::new(move |prev: Option<()>| {
        let (s, v) = sv.get();
        let hue_value = hue.get();
        if prev.is_none() {
            let hsv = match value.get_untracked() {
                Color::RGB(rgb) => rgb.into_color(),
                Color::HSV(hsv) => hsv,
                Color::HSL(hsl) => hsl.into_color(),
            };
            let Hsv {
                hue: h,
                saturation: s,
                value: v,
                ..
            } = hsv;
            hue.set(h.into_inner());
            sv.set((s.into(), v.into()))
        } else {
            value.update(|color| {
                let new_hsv: Hsv = Hsv::new(hue_value, s, v);
                match color {
                    Color::RGB(rgb) => *rgb = new_hsv.into_color(),
                    Color::HSV(hsv) => *hsv = new_hsv,
                    Color::HSL(hsl) => *hsl = new_hsv.into_color(),
                }
            });
        }
    });

    let is_show_popover = RwSignal::new(false);
    let trigger_ref = NodeRef::<html::Div>::new();
    let popover_ref = NodeRef::<html::Div>::new();
    let show_popover = move |_| {
        is_show_popover.set(true);
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let timer = window_event_listener(ev::click, move |ev| {
            let Some(popovel_el) = popover_ref.get_untracked() else {
                return;
            };
            let Some(trigger_el) = trigger_ref.get_untracked() else {
                return;
            };
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                if current_el == **popovel_el || current_el == **trigger_el {
                    return;
                }
                el = current_el.parent_element();
            }
            is_show_popover.set(false);
        });
        on_cleanup(move || timer.remove());
    }

    view! {
        <Binder>
            <div
                class=class_list!["thaw-color-picker-trigger", class]
                on:click=show_popover
                node_ref=trigger_ref
            >
                <div class="thaw-color-picker-trigger__content" style=move || style.get()>
                    {move || label.get()}
                </div>
            </div>
            <Follower slot show=is_show_popover placement=FollowerPlacement::BottomStart>
                <CSSTransition
                    name="fade-in-scale-up-transition"
                    appear=is_show_popover.get_untracked()
                    show=is_show_popover
                    let:display
                >
                    <div
                        class="thaw-config-provider thaw-color-picker-popover"
                        node_ref=popover_ref
                        style=move || display.get().unwrap_or_default()
                        data-thaw-id=config_provider.id()
                    >

                        <ColorPanel hue=hue.read_only() sv />
                        <HueSlider hue />
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[component]
fn ColorPanel(hue: ReadSignal<f32>, sv: RwSignal<(f32, f32)>) -> impl IntoView {
    let panel_ref = NodeRef::<html::Div>::new();
    let mouse = StoredValue::new(Vec::<WindowListenerHandle>::new());

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
                    format!("{:.2}", v).parse::<f32>().unwrap()
                };
                let s = if s > 1.0 {
                    1.0
                } else if s < 0.0 {
                    0.0
                } else {
                    format!("{:.2}", s).parse::<f32>().unwrap()
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
        <div class="thaw-color-picker-popover__panel" node_ref=panel_ref on:mousedown=on_mouse_down>
            <div
                class="thaw-color-picker-popover__layer"
                style:background-image=move || {
                    format!("linear-gradient(90deg, white, hsl({}, 100%, 50%))", hue.get())
                }
            ></div>
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
            ></div>
        </div>
    }
}

#[component]
fn HueSlider(hue: RwSignal<f32>) -> impl IntoView {
    let rail_ref = NodeRef::<html::Div>::new();
    let mouse = StoredValue::new(Vec::<WindowListenerHandle>::new());

    let on_mouse_down = move |ev| {
        let cb = move |ev: ev::MouseEvent| {
            if let Some(rail) = rail_ref.get_untracked() {
                let rect = rail.get_bounding_client_rect();
                let ev_x = f64::from(ev.x());
                let value = (ev_x - rect.left() - 6.0) / (rect.width() - 12.0) * 359.0;
                let value = if value < 0.0 {
                    0.0
                } else if value > 359.0 {
                    359.0
                } else {
                    value.round().to_string().parse::<f32>().unwrap()
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
        <div class="thaw-color-picker-slider" node_ref=rail_ref on:mousedown=on_mouse_down>
            <div
                class="thaw-color-picker-slider__handle"
                style=move || format!("left: calc({}% - 6px)", f32::from(hue.get()) / 359.0 * 100.0)
            ></div>
        </div>
    }
}
