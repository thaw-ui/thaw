use super::super::SliderInjection;
use leptos::{context::Provider, ev, html, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn RangeSlider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional, into)] value: Model<(f64, f64)>,
    /// Min value of the slider.
    #[prop(default = 0f64.into(), into)]
    min: Signal<f64>,
    /// Max value of the slider.
    #[prop(default = 100f64.into(), into)]
    max: Signal<f64>,
    /// The step in which value is incremented.
    #[prop(optional, into)]
    step: MaybeProp<f64>,
    /// Render the Slider in a vertical orientation, smallest value on the bottom.
    #[prop(optional, into)]
    vertical: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("range-slider", include_str!("./range-slider.css"));

    let slider_ref = NodeRef::<html::Div>::new();
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
        let mut css_vars = style.get().unwrap_or_default();

        if vertical.get() {
            css_vars.push_str(";--thaw-slider--direction: 0deg;");
        } else {
            css_vars.push_str(";--thaw-slider--direction: 90deg;");
        }

        if let Some(step) = step.get() {
            if step > 0.0 {
                let max = max.get();
                let min = min.get();

                css_vars.push_str(&format!(
                    ";--thaw-range-slider--steps-percent: {:.2}%;",
                    step * 100.0 / (max - min)
                ));
            }
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
        if let Some(slider) = slider_ref.get_untracked() {
            let min = min.get_untracked();
            let max = max.get_untracked();

            let rect = slider.get_bounding_client_rect();
            let percentage = if vertical.get_untracked() {
                let ev_y = f64::from(e.y());
                let slider_height = rect.height();
                (slider_height + rect.y() - ev_y) / slider_height * (max - min)
            } else {
                let ev_x = f64::from(e.x());
                (ev_x - rect.x()) / rect.width() * (max - min)
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
            if let Some(slider_el) = slider_ref.get_untracked() {
                let min = min.get_untracked();
                let max = max.get_untracked();

                let slider_rect = slider_el.get_bounding_client_rect();
                let percentage = if vertical.get_untracked() {
                    let ev_y = f64::from(e.y());
                    let slider_y = slider_rect.y();
                    let slider_height = slider_rect.height();

                    let length = if ev_y < slider_y {
                        0.0
                    } else if ev_y > slider_y + slider_height {
                        slider_height
                    } else {
                        slider_y + slider_height - ev_y
                    };

                    length / slider_height * (max - min)
                } else {
                    let ev_x = f64::from(e.x());
                    let slider_x = slider_rect.x();
                    let slider_width = slider_rect.width();

                    let length = if ev_x < slider_x {
                        0.0
                    } else if ev_x > slider_x + slider_width {
                        slider_width
                    } else {
                        ev_x - slider_x
                    };

                    length / slider_width * (max - min)
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
        left_mousemove.set_value(true);
        on_mousemove();
    };

    let on_right_mousedown = move |_| {
        right_mousemove.set_value(true);
        on_mousemove();
    };

    Owner::on_cleanup(cleanup_event);

    view! {
        <div
            class=class_list![
                "thaw-range-slider",
                move || format!("thaw-range-slider--{}", if vertical.get() { "vertical" } else { "horizontal" }),
                class
            ]
            on:click=on_click
            node_ref=slider_ref
            style=css_vars
        >
            <div class="thaw-range-slider__rail" style=rail_css_vars></div>
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
