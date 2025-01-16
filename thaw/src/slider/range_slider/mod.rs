use leptos::{ev, html, logging, prelude::*};
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn RangeSlider(
    #[prop(optional, into)] class: MaybeProp<String>,
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

        if left < min {
            left = min;
        } else if left > max {
            left = max;
        }

        if right < min {
            right = min;
        } else if right > max {
            right = max;
        }

        (left, right)
    });

    let left_css_vars = move || {
        format!(
            "--thaw-range-slider--progress: {:.2}%;",
            current_value.get().0
        )
    };
    let right_css_vars = move || {
        format!(
            "--thaw-range-slider--progress: {:.2}%;",
            current_value.get().1
        )
    };
    let rail_css_vars = move || {
        let (left, right) = current_value.get();
        let (left, right) = if left > right {
            (right, left)
        } else {
            (left, right)
        };
        format!("--thaw-range-slider--left-progress: {left:.2}%; --thaw-range-slider--right-progress: {right:.2}%;")
    };

    let on_click = move |e: web_sys::MouseEvent| {
        if let Some(slider) = slider_ref.get_untracked() {
            let rect = slider.get_bounding_client_rect();
            let ev_x = f64::from(e.x());
            let min = min.get_untracked();
            let max = max.get_untracked();

            let percentage = (ev_x - rect.x()) / rect.width() * (max - min);
            let (left, right) = current_value.get();

            let left_diff = (left - percentage).abs();
            let right_diff = (right - percentage).abs();

            if left_diff <= right_diff {
                value.set((percentage, right));
            } else {
                value.set((left, percentage));
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
                let slider_rect = slider_el.get_bounding_client_rect();
                let slider_width = slider_rect.width();
                let slider_x = slider_rect.x();
                let ev_x = f64::from(e.x());
                let length = if ev_x < slider_x {
                    0.0
                } else if ev_x > slider_x + slider_width {
                    slider_width
                } else {
                    ev_x - slider_x
                };
                let min = min.get_untracked();
                let max = max.get_untracked();
                let percentage = length / slider_width * (max - min);

                logging::log!("{percentage}");

                if left_mousemove.get_value() {
                    value.set((percentage, current_value.get_untracked().1));
                    return;
                } else if right_mousemove.get_value() {
                    value.set((current_value.get_untracked().0, percentage));
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
        <div class=class_list!["thaw-range-slider", class] on:click=on_click node_ref=slider_ref>
            <div class="thaw-range-slider__rail" style=rail_css_vars></div>
            <div class="thaw-range-slider__thumb" style=left_css_vars on:mousedown=on_left_mousedown></div>
            <div class="thaw-range-slider__thumb" style=right_css_vars on:mousedown=on_right_mousedown></div>
        </div>
    }
}
