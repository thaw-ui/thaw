mod slider_label;

pub use slider_label::SliderLabel;

use leptos::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};
use web_sys::DomRect;

#[component]
pub fn Slider(
    #[prop(optional, into)] value: Model<f64>,
    #[prop(default = MaybeSignal::Static(100f64), into)] max: MaybeSignal<f64>,
    #[prop(optional, into)] step: MaybeSignal<f64>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("slider", include_str!("./slider.css"));

    let percentage = create_memo(move |_| {
        if value.get() < 0.0 || max.get() <= 0.0 {
            0f64
        } else if value.get() >= max.get() {
            100f64
        } else {
            value.get() / max.get() * 100.0
        }
    });

    let do_update_value = move |mut val| {
        let step = step.get_untracked();
        if step > 0.0 {
            let result: f64 = val / step;
            if result.fract() != 0.0 {
                let prev_multiple = (result.floor() * step).abs();
                let mut next_multiple = (result.ceil() * step).abs();
                let max = max.get_untracked();
                if next_multiple > max {
                    next_multiple = max;
                }
                if val - prev_multiple > next_multiple - val {
                    val = next_multiple;
                } else {
                    val = prev_multiple;
                }
            }
        }
        value.set(val);
    };

    let rail_ref = create_node_ref::<html::Div>();
    let (mouse_move_value, set_mouse_move_value) = create_signal::<Option<f64>>(None);
    let (is_mouse_move, set_mouse_move) = create_signal(false);
    let on_mouse_down = move |_| {
        set_mouse_move.set(true);
    };

    let check_value_and_update = move |ev_x: f64, rect: DomRect| {
        if ev_x <= rect.x() {
            set_mouse_move_value.set(Some(0.0));
        } else if ev_x >= rect.x() + rect.width() {
            set_mouse_move_value.set(Some(max.get()));
        } else {
            set_mouse_move_value.set(Some(((ev_x - rect.x()) / rect.width()) * max.get()));
        }
        if let Some(value) = mouse_move_value.get_untracked() {
            do_update_value(value);
        }
    };

    let on_mouse_click = move |ev: ev::MouseEvent| {
        if let Some(rail) = rail_ref.get_untracked() {
            let rect = rail.get_bounding_client_rect();
            let ev_x = f64::from(ev.x());
            check_value_and_update(ev_x, rect);
        };
    };

    let on_mouse_up = window_event_listener(ev::mouseup, move |_| {
        set_mouse_move.set(false);
    });

    on_cleanup(move || on_mouse_up.remove());

    let on_mouse_move = window_event_listener(ev::mousemove, move |ev| {
        if is_mouse_move.get_untracked() {
            if let Some(rail) = rail_ref.get_untracked() {
                let rect = rail.get_bounding_client_rect();
                let ev_x = f64::from(ev.x());
                check_value_and_update(ev_x, rect);
            };
        }
    });
    on_cleanup(move || on_mouse_move.remove());

    view! {
        <div
            class=class_list!["thaw-slider", class.map(| c | move || c.get())]
            style=move || format!("--thaw-slider--direction: 90deg;--thaw-slider--progress: {:.2}%", value.get())
            on:click=on_mouse_click
        >
            <input type="range" class="thaw-slider__input"/>
            <div class="thaw-slider__rail" ref=rail_ref>
            </div>
            <div class="thaw-slider__thumb">
            </div>
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
            <div
                on:mousedown=on_mouse_down
                class="thaw-slider-handle"
                style=move || { format!("left: {}%", percentage.get()) }
            ></div>

        </div>
    }
}
