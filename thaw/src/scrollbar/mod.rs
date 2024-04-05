mod theme;

pub use theme::ScrollbarTheme;

use crate::{use_theme, Theme};
use leptos::{leptos_dom::helpers::WindowListenerHandle, *};
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Scrollbar(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: Option<MaybeSignal<String>>,
    #[prop(default = 8)] size: u8,
    children: Children,
) -> impl IntoView {
    mount_style("scrollbar", include_str!("./scrollbar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-scrollbar-background-color: {};",
                theme.scrollbar.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-scrollbar-background-color-hover: {};",
                theme.scrollbar.background_color_hover
            ));
            css_vars.push_str(&format!("--thaw-scrollbar-size: {}px;", size));
        });
        css_vars
    });

    let container_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<html::Div>::new();
    let x_track_ref = NodeRef::<html::Div>::new();
    let y_track_ref = NodeRef::<html::Div>::new();
    let is_show_x_thumb = RwSignal::new(false);
    let is_show_y_thumb = RwSignal::new(false);
    let x_track_width = RwSignal::new(0);
    let y_track_height = RwSignal::new(0);
    let container_width = RwSignal::new(0);
    let container_height = RwSignal::new(0);
    let container_scroll_top = RwSignal::new(0);
    let container_scroll_left = RwSignal::new(0);
    let content_width = RwSignal::new(0);
    let content_height = RwSignal::new(0);
    let thumb_status = StoredValue::new(None::<ThumbStatus>);

    let x_thumb_width = Memo::new(move |_| {
        let content_width = f64::from(content_width.get());
        let x_track_width = f64::from(x_track_width.get());
        let container_width = f64::from(container_width.get());
        if content_width <= 0.0 {
            return 0.0;
        }
        x_track_width * container_width / content_width
    });
    let x_thumb_left = Memo::new(move |_| {
        let x_track_width = f64::from(x_track_width.get());
        let x_thumb_width = f64::from(x_thumb_width.get());
        if x_track_width == x_thumb_width {
            is_show_x_thumb.set(false);
            return 0.0;
        }

        let container_width = f64::from(container_width.get());
        let container_scroll_left = f64::from(container_scroll_left.get());
        let content_width = f64::from(content_width.get());

        let diff = content_width - container_width;
        if diff <= 0.0 {
            0.0
        } else {
            (container_scroll_left / diff) * (x_track_width - x_thumb_width)
        }
    });

    let y_thumb_height = Memo::new(move |_| {
        let content_height = f64::from(content_height.get());
        let y_track_height = f64::from(y_track_height.get());
        let container_height = f64::from(container_height.get());
        if content_height <= 0.0 {
            return 0.0;
        }
        y_track_height * container_height / content_height
    });
    let y_thumb_top = Memo::new(move |_| {
        let y_track_height = f64::from(y_track_height.get());
        let y_thumb_height = f64::from(y_thumb_height.get());
        if y_track_height == y_thumb_height {
            is_show_y_thumb.set(false);
            return 0.0;
        }

        let container_height = f64::from(container_height.get());
        let container_scroll_top = f64::from(container_scroll_top.get());
        let content_height = f64::from(content_height.get());

        let diff = content_height - container_height;
        if diff <= 0.0 {
            0.0
        } else {
            (container_scroll_top / diff) * (y_track_height - y_thumb_height)
        }
    });

    let sync_scroll_state = move || {
        if let Some(el) = container_ref.get_untracked() {
            container_scroll_top.set(el.scroll_top());
            container_scroll_left.set(el.scroll_left());
        }
    };
    let sync_position_state = move || {
        if let Some(el) = container_ref.get_untracked() {
            container_width.set(el.offset_width());
            container_height.set(el.offset_height());
        }
        if let Some(el) = content_ref.get_untracked() {
            content_width.set(el.offset_width());
            content_height.set(el.offset_height());
        }
        if let Some(el) = x_track_ref.get() {
            x_track_width.set(el.offset_width());
        }
        if let Some(el) = y_track_ref.get() {
            y_track_height.set(el.offset_height());
        }
    };
    let on_mouseenter = move |_| {
        is_show_x_thumb.set(true);
        is_show_y_thumb.set(true);
        thumb_status.update_value(|thumb_status| {
            if thumb_status.is_some() {
                *thumb_status = Some(ThumbStatus::Enter);
            }
        });
        sync_position_state();
        sync_scroll_state();
    };
    let on_mouseleave = move |_| {
        thumb_status.update_value(|thumb_status| {
            if thumb_status.is_some() {
                *thumb_status = Some(ThumbStatus::DelayLeave);
            } else {
                is_show_y_thumb.set(false);
                is_show_x_thumb.set(false);
            }
        });
    };

    let on_scroll = move |_| {
        sync_scroll_state();
    };

    let x_trumb_mousemove_handle = StoredValue::new(None::<WindowListenerHandle>);
    let x_trumb_mouseup_handle = StoredValue::new(None::<WindowListenerHandle>);
    let memo_x_left = StoredValue::new(0);
    let memo_mouse_x = StoredValue::new(0);
    let on_x_thumb_mousedown = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        let handle = window_event_listener(ev::mousemove, move |e| {
            let container_width = container_width.get();
            let content_width = content_width.get();
            let x_track_width = x_track_width.get();
            let x_thumb_width = x_thumb_width.get() as i32;

            let x_diff = e.client_x() - memo_mouse_x.get_value();
            let to_scroll_left_upper_bound = content_width - container_width;
            let scroll_left =
                (x_diff * to_scroll_left_upper_bound) / (x_track_width - x_thumb_width);

            let mut to_scroll_left = memo_x_left.get_value() + scroll_left;
            to_scroll_left = to_scroll_left.min(to_scroll_left_upper_bound);
            to_scroll_left = to_scroll_left.max(0);

            if let Some(el) = container_ref.get_untracked() {
                el.set_scroll_left(to_scroll_left);
            }
        });
        x_trumb_mousemove_handle.set_value(Some(handle));
        let handle = window_event_listener(ev::mouseup, move |_| {
            x_trumb_mousemove_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            x_trumb_mouseup_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            thumb_status.update_value(|thumb_status| {
                if let Some(status) = thumb_status.take() {
                    if status == ThumbStatus::DelayLeave {
                        is_show_x_thumb.set(false);
                        is_show_y_thumb.set(false);
                    }
                }
            });
        });
        x_trumb_mouseup_handle.set_value(Some(handle));
        memo_x_left.set_value(container_scroll_left.get());
        memo_mouse_x.set_value(e.client_x());
        thumb_status.set_value(Some(ThumbStatus::Enter));
    };

    let y_trumb_mousemove_handle = StoredValue::new(None::<WindowListenerHandle>);
    let y_trumb_mouseup_handle = StoredValue::new(None::<WindowListenerHandle>);
    let memo_y_top = StoredValue::new(0);
    let memo_mouse_y = StoredValue::new(0);
    let on_y_thumb_mousedown = move |e: ev::MouseEvent| {
        e.prevent_default();
        e.stop_propagation();
        let handle = window_event_listener(ev::mousemove, move |e| {
            let container_height = container_height.get();
            let content_height = content_height.get();
            let y_track_height = y_track_height.get();
            let y_thumb_height = y_thumb_height.get() as i32;

            let y_diff = e.client_y() - memo_mouse_y.get_value();
            let to_scroll_top_upper_bound = content_height - container_height;
            let scroll_top =
                (y_diff * to_scroll_top_upper_bound) / (y_track_height - y_thumb_height);

            let mut to_scroll_top = memo_y_top.get_value() + scroll_top;
            to_scroll_top = to_scroll_top.min(to_scroll_top_upper_bound);
            to_scroll_top = to_scroll_top.max(0);

            if let Some(el) = container_ref.get_untracked() {
                el.set_scroll_top(to_scroll_top);
            }
        });
        y_trumb_mousemove_handle.set_value(Some(handle));
        let handle = window_event_listener(ev::mouseup, move |_| {
            y_trumb_mousemove_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            y_trumb_mouseup_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
            thumb_status.update_value(|thumb_status| {
                if let Some(status) = thumb_status.take() {
                    if status == ThumbStatus::DelayLeave {
                        is_show_x_thumb.set(false);
                        is_show_y_thumb.set(false);
                    }
                }
            });
        });
        y_trumb_mouseup_handle.set_value(Some(handle));
        memo_y_top.set_value(container_scroll_top.get());
        memo_mouse_y.set_value(e.client_y());
        thumb_status.set_value(Some(ThumbStatus::Enter));
    };

    view! {
        <div
            class=class_list!["thaw-scrollbar", class.map(| c | move || c.get())]
            style=move || {
                format!("{}{}", css_vars.get(), style.as_ref().map(|s| s.get()).unwrap_or_default())
            }

            on:mouseenter=on_mouseenter
            on:mouseleave=on_mouseleave
        >

            <div class="thaw-scrollbar__container" ref=container_ref on:scroll=on_scroll>
                <div class="thaw-scrollbar__content" style="width: fit-content;" ref=content_ref>{children()}</div>
            </div>
            <div class="thaw-scrollbar__track--vertical" ref=y_track_ref>
                <div
                    class="thaw-scrollabr__thumb"
                    style:display=move || (!is_show_y_thumb.get()).then_some("none")
                    style:height=move || format!("{}px", y_thumb_height.get())
                    style:top=move || format!("{}px", y_thumb_top.get())
                    on:mousedown=on_y_thumb_mousedown
                ></div>
            </div>
            <div class="thaw-scrollbar__track--horizontal" ref=x_track_ref>
                <div
                    class="thaw-scrollabr__thumb"
                    style:display=move || (!is_show_x_thumb.get()).then_some("none")
                    style:width=move || format!("{}px", x_thumb_width.get())
                    style:left=move || format!("{}px", x_thumb_left.get())
                    on:mousedown=on_x_thumb_mousedown
                ></div>
            </div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
enum ThumbStatus {
    Enter,
    DelayLeave,
}
