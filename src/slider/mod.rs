mod theme;

use crate::{
    theme::use_theme,
    utils::{maybe_rw_signal::MaybeRwSignal, mount_style::mount_style},
    Theme,
};
use leptos::*;
pub use theme::SliderTheme;

#[component]
pub fn Slider(
    #[prop(optional, into)] value: MaybeRwSignal<f64>,
    #[prop(default = MaybeSignal::Static(100f64), into)] max: MaybeSignal<f64>,
) -> impl IntoView {
    mount_style("slider", include_str!("./slider.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--melt-background-color: {};",
                &theme.slider.background_color
            ));
            css_vars.push_str(&format!(
                "--melt-background-color-fill: {};",
                &theme.common.color_primary
            ));
        });

        css_vars
    });

    let percentage = create_memo(move |_| {
        if value.get() < 0.0 || max.get() <= 0.0 {
            0f64
        } else if value.get() >= max.get() {
            100f64
        } else {
            value.get() / max.get() * 100.0
        }
    });

    let do_update_value = move |val| {
        value.set(val);
    };

    let rail_ref = create_node_ref::<html::Div>();
    let (mouse_move_value, set_mouse_move_value) = create_signal::<Option<f64>>(None);
    let (is_mouse_move, set_mouse_move) = create_signal(false);
    let on_mouse_down = move |_| {
        set_mouse_move.set(true);
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
        }
    });
    on_cleanup(move || on_mouse_move.remove());

    view! {
        <div class="melt-slider" style=move || css_vars.get()>
            <div class="melt-slider-rail" ref=rail_ref>
                <div
                    class="melt-slider-rail__fill"
                    style=move || format!("width: {}%", percentage.get())
                ></div>
            </div>
            <div
                on:mousedown=on_mouse_down
                class="melt-slider-handle"
                style=move || {
                    format!(
                        "left: {}%; transform: translateX(-{}%)",
                        percentage.get(),
                        percentage.get(),
                    )
                }
            >
            </div>
        </div>
    }
}
