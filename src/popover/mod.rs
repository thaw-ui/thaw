use crate::components::{Binder, Follower, FollowerPlacement};
use leptos::*;

#[slot]
pub struct PopoverTrigger {
    children: Children,
}

#[component]
pub fn Popover(
    popover_trigger: PopoverTrigger,
    #[prop(optional)] placement: PopoverPlacement,
    children: Children,
) -> impl IntoView {
    let target_ref = create_node_ref::<html::Div>();
    let is_show_popover = create_rw_signal(false);
    let on_mouse_enter = move |_| {
        is_show_popover.set(true);
    };
    let on_mouse_leave = move |_| {
        is_show_popover.set(false);
    };
    view! {
        <Binder target_ref>
            <div ref=target_ref on:mouseenter=on_mouse_enter on:mouseleave=on_mouse_leave>
                {(popover_trigger.children)()}
            </div>
            <Follower slot show=is_show_popover placement>
                <div>
                    {children()}
                </div>
            </Follower>
        </Binder>
    }
}

#[derive(Default)]
pub enum PopoverPlacement {
    #[default]
    Top,
    // Bottom,
    // Left,
    // Right,
    // TopStart,
    // TopEnd,
    // LeftStart,
    // LeftEnd,
    // RightStart,
    // RightEnd,
    BottomStart,
    // BottomEnd,
}

impl From<PopoverPlacement> for FollowerPlacement {
    fn from(value: PopoverPlacement) -> Self {
        match value {
            PopoverPlacement::Top => Self::Top,
            PopoverPlacement::BottomStart => Self::BottomStart,
        }
    }
}
