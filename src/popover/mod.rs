mod theme;

use crate::{
    components::{Binder, Follower, FollowerPlacement},
    use_theme,
    utils::mount_style,
    Theme,
};
use leptos::*;
pub use theme::PopoverTheme;

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
    mount_style("popover", include_str!("./popover.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.time_picker.panel_background_color
            ));
        });
        css_vars
    });
    let target_ref = create_node_ref::<html::Div>();
    let is_show_popover = create_rw_signal(false);
    let on_mouse_enter = move |_| {
        is_show_popover.set(true);
    };
    let on_mouse_leave = move |_| {
        is_show_popover.set(false);
    };
    let placement_str = placement.as_str();
    view! {
        <Binder target_ref>
            <div ref=target_ref on:mouseenter=on_mouse_enter on:mouseleave=on_mouse_leave class="thaw-popover-trigger">
                {(popover_trigger.children)()}
            </div>
            <Follower slot show=is_show_popover placement>
                <div class=format!("thaw-popover thaw-popover--{placement_str}") style=move || css_vars.get()>
                    {children()}
                    <div class="thaw-popover__angle-container">
                        <div class="thaw-popover__angle"></div>
                    </div>
                </div>
            </Follower>
        </Binder>
    }
}

#[derive(Default)]
pub enum PopoverPlacement {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    // TopStart,
    // TopEnd,
    // LeftStart,
    // LeftEnd,
    // RightStart,
    // RightEnd,
    BottomStart,
    // BottomEnd,
}

impl PopoverPlacement {
    fn as_str(&self) -> &'static str {
        match self {
            PopoverPlacement::Top => "top",
            PopoverPlacement::Bottom => "bottom",
            PopoverPlacement::Left => "left",
            PopoverPlacement::Right => "right",
            PopoverPlacement::BottomStart => "bottom-start",
        }
    }
}

impl From<PopoverPlacement> for FollowerPlacement {
    fn from(value: PopoverPlacement) -> Self {
        match value {
            PopoverPlacement::Top => Self::Top,
            PopoverPlacement::Bottom => Self::Bottom,
            PopoverPlacement::Left => Self::Left,
            PopoverPlacement::Right => Self::Right,
            PopoverPlacement::BottomStart => Self::BottomStart,
        }
    }
}
