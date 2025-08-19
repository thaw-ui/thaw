use super::super::SliderInjection;
use leptos::{context::Provider, ev, html, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn RangeSlider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    /// Whether the slider is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// The current value of the controlled Slider.
    #[prop(optional, into)]
    value: Model<(f64, f64)>,
    /// Min value of the slider.
    #[prop(default = 0f64.into(), into)]
    min: Signal<f64>,
    /// Max value of the slider.
    #[prop(default = 100f64.into(), into)]
    max: Signal<f64>,
    /// The step in which value is incremented.
    #[prop(optional, into)]
    step: MaybeProp<f64>,
    /// Whether to display breakpoints.
    #[prop(default = true.into(), into)]
    show_stops: Signal<bool>,
    /// Render the Slider in a vertical orientation, smallest value on the bottom.
    #[prop(optional, into)]
    vertical: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("range-slider", include_str!("./range-slider.css"));

    let rail_ref = NodeRef::<html::Div>::new();
    let left_mousemove = StoredValue::new(false);
    let right_mousemove = StoredValue::new(false);
    let mousemove_handle = StoredValue::new(None::<WindowListenerHandle>);
    let mouseup_handle = StoredValue::new(None::<WindowListenerHandle>);
    let current_value = Memo::new(move |_| {
        let (mut left, mut right) = value.get();
        let min = min.get();
        let max = max.get();
        let step = step.get().unwrap_or_default();

        left = closest_multiple(left, step, min, max);
        right = closest_multiple(right, step, min, max);

        (left, right)
    });

    let left_progress = Memo::new(move |_| {
        let value = current_value.get().0;
        let min = min.get();
        let max = max.get();

        value / (max - min) * 100.0
    });

    let right_progress = Memo::new(move |_| {
        let value = current_value.get().1;
        let min = min.get();
        let max = max.get();

        value / (max - min) * 100.0
    });

    let css_vars = move || {
        let mut css_vars = String::new();

        if vertical.get() {
            css_vars.push_str("--thaw-slider--direction: 0deg;");
        } else {
            css_vars.push_str("--thaw-slider--direction: 90deg;");
        }

        if let Some(step) = step.get() {
            if step > 0.0 && show_stops.get() {
                let max = max.get();
                let min = min.get();

                css_vars.push_str(&format!(
                    "--thaw-range-slider--steps-percent: {:.2}%;",
                    step * 100.0 / (max - min)
                ));
            }
        }

        if let Some(style) = style.get() {
            css_vars.push_str(&style);
        }

        css_vars
    };

    let rail_css_vars = move || {
        let left = left_progress.get();
        let right = right_progress.get();
        let (left, right) = if left > right {
            (right, left)
        } else {
            (left, right)
        };
        format!("--thaw-range-slider--left-progress: {left:.2}%; --thaw-range-slider--right-progress: {right:.2}%;")
    };

    let update_value = move |left, right| {
        let min = min.get_untracked();
        let max = max.get_untracked();
        let step = step.get_untracked().unwrap_or_default();

        value.set((
            closest_multiple(left, step, min, max),
            closest_multiple(right, step, min, max),
        ));
    };

    let on_click = move |e: web_sys::MouseEvent| {
        if disabled.get() {
            return;
        }
        if let Some(rail_el) = rail_ref.get_untracked() {
            let min = min.get_untracked();
            let max = max.get_untracked();

            let rail_rect = rail_el.get_bounding_client_rect();
            let percentage = if vertical.get_untracked() {
                let ev_y = f64::from(e.y());
                let rail_height = rail_rect.height();
                (rail_height + rail_rect.y() - ev_y) / rail_height * (max - min)
            } else {
                let ev_x = f64::from(e.x());
                (ev_x - rail_rect.x()) / rail_rect.width() * (max - min)
            };

            let (left, right) = current_value.get();
            let left_diff = (left - percentage).abs();
            let right_diff = (right - percentage).abs();

            if left_diff <= right_diff {
                update_value(percentage, right);
            } else {
                update_value(left, percentage);
            }
        };
    };

    let cleanup_event = move || {
        mousemove_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });

        mouseup_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    };

    let on_mousemove = move || {
        let mousemove = window_event_listener(ev::mousemove, move |e| {
            if let Some(rail_el) = rail_ref.get_untracked() {
                let min = min.get_untracked();
                let max = max.get_untracked();

                let rail_rect = rail_el.get_bounding_client_rect();
                let percentage = if vertical.get_untracked() {
                    let ev_y = f64::from(e.y());
                    let rail_y = rail_rect.y();
                    let rail_height = rail_rect.height();

                    let length = if ev_y < rail_y {
                        rail_height
                    } else if ev_y > rail_y + rail_height {
                        0.0
                    } else {
                        rail_y + rail_height - ev_y
                    };

                    length / rail_height * (max - min)
                } else {
                    let ev_x = f64::from(e.x());
                    let rail_x = rail_rect.x();
                    let rail_width = rail_rect.width();

                    let length = if ev_x < rail_x {
                        0.0
                    } else if ev_x > rail_x + rail_width {
                        rail_width
                    } else {
                        ev_x - rail_x
                    };

                    length / rail_width * (max - min)
                };

                if left_mousemove.get_value() {
                    update_value(percentage, current_value.get_untracked().1);
                    return;
                } else if right_mousemove.get_value() {
                    update_value(current_value.get_untracked().0, percentage);
                    return;
                }
            }
            cleanup_event();
        });

        let mouseup = window_event_listener(ev::mouseup, move |_| {
            left_mousemove.set_value(false);
            right_mousemove.set_value(false);
            cleanup_event();
        });

        mousemove_handle.set_value(Some(mousemove));
        mouseup_handle.set_value(Some(mouseup));
    };

    let on_left_mousedown = move |_| {
        if disabled.get() {
            return;
        }
        left_mousemove.set_value(true);
        on_mousemove();
    };

    let on_right_mousedown = move |_| {
        if disabled.get() {
            return;
        }
        right_mousemove.set_value(true);
        on_mousemove();
    };

    Owner::on_cleanup(cleanup_event);

    view! {
        <div
            class=class_list![
                "thaw-range-slider",
                ("thaw-range-slider--disabled", move || disabled.get()),
                move || format!("thaw-range-slider--{}", if vertical.get() { "vertical" } else { "horizontal" }),
                class
            ]
            on:click=on_click
            style=css_vars
        >
            <div class="thaw-range-slider__rail" style=rail_css_vars node_ref=rail_ref></div>
            <div
                class="thaw-range-slider__thumb"
                style=move || format!("--thaw-range-slider--progress: {:.2}%;", left_progress.get())
                on:mousedown=on_left_mousedown
            ></div>
            <div
                class="thaw-range-slider__thumb"
                style=move || {
                    format!("--thaw-range-slider--progress: {:.2}%;", right_progress.get())
                }
                on:mousedown=on_right_mousedown
            ></div>
            <OptionComp value=children let:children>
                <Provider value=SliderInjection {
                    max,
                    min,
                    vertical,
                }>
                    <div class="thaw-range-slider__datalist">{children()}</div>
                </Provider>
            </OptionComp>
        </div>
    }
}

fn closest_multiple(mut value: f64, multiple: f64, min: f64, max: f64) -> f64 {
    if value < min {
        return min;
    }

    if multiple <= 0.0 {
        return if value > max { max } else { value };
    }

    let quotient = (value - min) / multiple;

    let lower_multiple = quotient.floor() * multiple + min;
    let upper_multiple = quotient.ceil() * multiple + min;

    value = if (value - lower_multiple).abs() <= (value - upper_multiple).abs() {
        lower_multiple
    } else {
        upper_multiple
    };

    while value > max {
        value -= multiple;
    }

    if value < min {
        min
    } else {
        value
    }
}
