use crate::ConfigInjection;
use leptos::{ev, html, leptos_dom::helpers::TimeoutHandle, prelude::*};
use std::time::Duration;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{add_event_listener, class_list, mount_style};

#[slot]
pub struct PopoverTrigger {
    #[prop(optional, into)]
    class: MaybeProp<String>,
    children: Children,
}

#[component]
pub fn Popover(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] trigger_type: PopoverTriggerType,
    popover_trigger: PopoverTrigger,
    #[prop(optional)] placement: PopoverPlacement,
    #[prop(optional, into)] appearance: Option<MaybeSignal<PopoverAppearance>>,
    children: Children,
) -> impl IntoView {
    mount_style("popover", include_str!("./popover.css"));
    let config_provider = ConfigInjection::use_();

    let popover_ref = NodeRef::<html::Div>::new();
    let target_ref = NodeRef::<html::Div>::new();
    let is_show_popover = RwSignal::new(false);
    let show_popover_handle = StoredValue::new(None::<TimeoutHandle>);

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
            if !is_show_popover.get_untracked() {
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
                if current_el == **popover_el {
                    return;
                }
                el = current_el.parent_element();
            }
            is_show_popover.set(false);
        });
        on_cleanup(move || handle.remove());
    }

    Effect::new(move |_| {
        let Some(target_el) = target_ref.get() else {
            return;
        };
        let handler = add_event_listener(target_el.into(), ev::click, move |event| {
            if trigger_type != PopoverTriggerType::Click {
                return;
            }
            event.stop_propagation();
            is_show_popover.update(|show| *show = !*show);
        });
        on_cleanup(move || handler.remove());
    });

    let PopoverTrigger {
        class: trigger_class,
        children: trigger_children,
    } = popover_trigger;

    view! {
        <Binder target_ref>
            <div
                class=class_list!["thaw-popover-trigger", trigger_class]
                node_ref=target_ref
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
                        class=class_list![
                            "thaw-config-provider thaw-popover-surface",
                            appearance.map(|appearance| move || format!("thaw-popover-surface--{}", appearance.get().as_str())),
                            class
                        ]
                        data-thaw-id=config_provider.id().clone()
                        style=move || display.get().unwrap_or_default()

                        node_ref=popover_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        {children()}
                        <div class="thaw-popover-surface__angle">
                        </div>
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Clone)]
pub enum PopoverAppearance {
    Brand,
    Inverted,
}

impl PopoverAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            PopoverAppearance::Brand => "brand",
            PopoverAppearance::Inverted => "inverted",
        }
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
