mod types;

pub use types::*;

use crate::ConfigInjection;
use leptos::{
    ev::{self, on},
    html,
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
    tachys::html::{class::class as tachys_class, node_ref::node_ref},
};
use std::time::Duration;
use thaw_components::{Binder, CSSTransition, Follower};
use thaw_utils::{add_event_listener, class_list, mount_style, BoxCallback};

#[component]
pub fn Popover<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Action that displays the popover.
    #[prop(optional)]
    trigger_type: PopoverTriggerType,
    /// The element or component that triggers popover.
    popover_trigger: PopoverTrigger<T>,
    /// Configures the position of the Popover.
    #[prop(optional)]
    position: PopoverPosition,
    /// A popover can appear styled with brand or inverted.
    /// When not specified, the default style is used.
    #[prop(optional, into)]
    appearance: MaybeProp<PopoverAppearance>,
    #[prop(optional, into)] size: Signal<PopoverSize>,
    #[prop(optional, into)] on_open: Option<BoxCallback>,
    #[prop(optional, into)] on_close: Option<BoxCallback>,
    children: Children,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    mount_style("popover", include_str!("./popover.css"));
    let config_provider = ConfigInjection::expect_context();

    let popover_ref = NodeRef::<html::Div>::new();
    let target_ref = NodeRef::<thaw_utils::Element>::new();
    let is_show_popover = RwSignal::new(false);
    let show_popover_handle = StoredValue::new(None::<TimeoutHandle>);

    if on_open.is_some() || on_close.is_some() {
        Effect::watch(
            move || is_show_popover.get(),
            move |is_shown, prev_is_shown, _| {
                if prev_is_shown != Some(is_shown) {
                    if *is_shown {
                        if let Some(on_open) = &on_open {
                            on_open();
                        }
                    } else {
                        if let Some(on_close) = &on_close {
                            on_close();
                        }
                    }
                }
            },
            false,
        );
    }

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
        let handler = add_event_listener(target_el, ev::click, move |event| {
            if trigger_type != PopoverTriggerType::Click {
                return;
            }
            event.stop_propagation();
            is_show_popover.update(|show| *show = !*show);
        });
        on_cleanup(move || handler.remove());
    });

    let PopoverTrigger {
        children: trigger_children,
    } = popover_trigger;

    view! {
        <Binder>
            {trigger_children
                .into_inner()()
                .into_inner()
                .add_any_attr(tachys_class(("thaw-popover-trigger", true)))
                .add_any_attr(
                    tachys_class(("thaw-popover-trigger--open", move || is_show_popover.get())),
                )
                .add_any_attr(node_ref(target_ref))
                .add_any_attr(on(ev::mouseenter, on_mouse_enter))
                .add_any_attr(on(ev::mouseleave, on_mouse_leave))}
            <Follower slot show=is_show_popover placement=position>
                <CSSTransition
                    name="popover-transition"
                    appear=is_show_popover.get_untracked()
                    show=is_show_popover
                    let:display
                >
                    <div
                        class=class_list![
                            "thaw-config-provider thaw-popover-surface",
                            move || format!("thaw-popover-surface--{}", size.get().as_str()),
                            move || appearance.get().map(|a| format!("thaw-popover-surface--{}", a.as_str())),
                            class
                        ]
                        data-thaw-id=config_provider.id()
                        style=move || display.get().unwrap_or_default()

                        node_ref=popover_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        {children()}
                        <div class="thaw-popover-surface__angle"></div>
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}
