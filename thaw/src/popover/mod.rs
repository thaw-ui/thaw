mod theme;

pub use theme::PopoverTheme;

use crate::{use_theme, Theme};
use leptos::{leptos_dom::helpers::TimeoutHandle, *};
use std::time::Duration;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{add_event_listener, class_list, mount_style, OptionalProp};

#[slot]
pub struct PopoverTrigger {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[component]
pub fn Popover(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] trigger_type: PopoverTriggerType,
    popover_trigger: PopoverTrigger,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional)] tooltip: bool,
    children: Children,
) -> impl IntoView {
    mount_style("popover", include_str!("./popover.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let background_color = if tooltip {
                &theme.popover.tooltip_background_color
            } else {
                &theme.popover.background_color
            };
            css_vars.push_str(&format!("--thaw-background-color: {};", background_color));
            let font_color = if tooltip {
                "#fff"
            } else {
                &theme.common.font_color
            };
            css_vars.push_str(&format!("--thaw-font-color: {};", font_color));
        });
        css_vars
    });
    let popover_ref = create_node_ref::<html::Div>();
    let target_ref = create_node_ref::<html::Div>();
    let is_show_popover = create_rw_signal(false);
    let show_popover_handle = store_value(None::<TimeoutHandle>);

    let on_mouse_enter = move |_| {
        if trigger_type != PopoverTriggerType::Hover {
            return;
        }
        show_popover_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        is_show_popover.set(true);
    };
    let on_mouse_leave = move |_| {
        if trigger_type != PopoverTriggerType::Hover {
            return;
        }
        show_popover_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || {
                    is_show_popover.set(false);
                },
                Duration::from_millis(100),
            )
            .ok();
        });
    };
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(ev::click, move |ev| {
            use leptos::wasm_bindgen::__rt::IntoJsResult;
            if trigger_type != PopoverTriggerType::Click {
                return;
            }
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                let Some(popover_el) = popover_ref.get_untracked() else {
                    break;
                };
                if current_el == ***popover_el {
                    return;
                }
                el = current_el.parent_element();
            }
            is_show_popover.set(false);
        });
        on_cleanup(move || handle.remove());
    }

    target_ref.on_load(move |target_el| {
        add_event_listener(target_el.into_any(), ev::click, move |event| {
            if trigger_type != PopoverTriggerType::Click {
                return;
            }
            event.stop_propagation();
            is_show_popover.update(|show| *show = !*show);
        });
    });
    let PopoverTrigger {
        class: trigger_class,
        children: trigger_children,
    } = popover_trigger;

    view! {
        <Binder target_ref>
            <div
                class=class_list!["thaw-popover-trigger", trigger_class.map(| c | move || c.get())]
                ref=target_ref
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {trigger_children()}
            </div>
            <Follower slot show=is_show_popover placement>
                <CSSTransition
                    node_ref=popover_ref
                    name="popover-transition"
                    appear=is_show_popover.get_untracked()
                    show=is_show_popover
                    let:display
                >
                    <div
                        class=if tooltip { "thaw-popover thaw-popover--tooltip" } else { "thaw-popover" }
                        style=move || {
                            display.get().map(|d| d.to_string()).unwrap_or_else(|| css_vars.get())
                        }

                        ref=popover_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        <div class=class.map(|c| move || c.get())>{children()}</div>
                        <div class="thaw-popover__angle-container">
                            <div class="thaw-popover__angle"></div>
                        </div>
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum PopoverTriggerType {
    #[default]
    Hover,
    Click,
}

impl Copy for PopoverTriggerType {}

#[derive(Default)]
pub enum PopoverPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}

impl From<PopoverPlacement> for FollowerPlacement {
    fn from(value: PopoverPlacement) -> Self {
        match value {
            PopoverPlacement::Top => Self::Top,
            PopoverPlacement::Bottom => Self::Bottom,
            PopoverPlacement::Left => Self::Left,
            PopoverPlacement::Right => Self::Right,
            PopoverPlacement::TopStart => Self::TopStart,
            PopoverPlacement::TopEnd => Self::TopEnd,
            PopoverPlacement::LeftStart => Self::LeftStart,
            PopoverPlacement::LeftEnd => Self::LeftEnd,
            PopoverPlacement::RightStart => Self::RightStart,
            PopoverPlacement::RightEnd => Self::RightEnd,
            PopoverPlacement::BottomStart => Self::BottomStart,
            PopoverPlacement::BottomEnd => Self::BottomEnd,
        }
    }
}
