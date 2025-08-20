mod types;

pub use types::*;

use leptos::{
    either::Either,
    ev::{self, on},
    html,
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
    tachys::html::{class::class as tachys_class, node_ref::node_ref},
};
use std::time::Duration;
use thaw_components::{Follower, FollowerArrow};
use thaw_utils::{class_list, mount_style, on_click_outside, BoxCallback};

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

    let popover_ref = NodeRef::<html::Div>::new();
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

    let PopoverTrigger {
        children: trigger_children,
    } = popover_trigger;
    let trigger_children = trigger_children.into_inner()()
        .into_inner()
        .add_any_attr(tachys_class(("thaw-popover-trigger", true)))
        .add_any_attr(tachys_class(("thaw-popover-trigger--open", move || {
            is_show_popover.get()
        })));

    let trigger_children = match trigger_type {
        PopoverTriggerType::Click => {
            let trigger_ref = NodeRef::<thaw_utils::Element>::new();
            on_click_outside(
                move || {
                    if !is_show_popover.get_untracked() {
                        return None;
                    }
                    let Some(trigger_el) = trigger_ref.get_untracked() else {
                        return None;
                    };
                    let Some(popover_el) = popover_ref.get_untracked() else {
                        return None;
                    };
                    Some(vec![popover_el.into(), trigger_el])
                },
                move || is_show_popover.set(false),
            );
            Either::Left(
                trigger_children
                    .add_any_attr(node_ref(trigger_ref))
                    .add_any_attr(on(ev::click, move |_| {
                        is_show_popover.update(|show| {
                            *show = !*show;
                        });
                    })),
            )
        }
        PopoverTriggerType::Hover => Either::Right(
            trigger_children
                .add_any_attr(on(ev::mouseenter, on_mouse_enter))
                .add_any_attr(on(ev::mouseleave, on_mouse_leave)),
        ),
    };

    let arrow_ref = NodeRef::<html::Div>::new();
    let edge_length = 1.414 * 8.0;
    let arrow_style = format!(
        "--thaw-positioning-arrow-height: {}px; --thaw-positioning-arrow-offset: {}px;",
        edge_length,
        (edge_length / 2.0) * -1.0
    );
    let arrow = FollowerArrow {
        safe_width: 4.0,
        width: edge_length / 2.0 + 1.0,
        height: edge_length / 2.0 + 2.0,
        node_ref: arrow_ref,
    };

    view! {
        <crate::_binder::Binder>
            {trigger_children} <Follower slot show=is_show_popover placement=position arrow=arrow>
                <div
                    class=class_list![
                        "thaw-popover-surface",
                        move || format!("thaw-popover-surface--{}", size.get().as_str()),
                        move || appearance.get().map(|a| format!("thaw-popover-surface--{}", a.as_str())),
                        class
                    ]

                    node_ref=popover_ref
                    on:mouseenter=on_mouse_enter
                    on:mouseleave=on_mouse_leave
                >
                    {children()}
                    <div
                        class="thaw-popover-surface__angle"
                        style=arrow_style
                        node_ref=arrow_ref
                    ></div>
                </div>
            </Follower>
        </crate::_binder::Binder>
    }
}
