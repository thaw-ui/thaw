mod theme;

use crate::{
    components::{Binder, Follower, FollowerPlacement},
    use_theme,
    utils::{dyn_classes, mount_style, ssr_class},
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
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] content_class: MaybeSignal<String>,
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
    let ssr_class = ssr_class(&class);
    view! {
        <Binder target_ref>
            <div
                class=ssr_class
                use:dyn_classes=class
                ref=target_ref
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
                class="thaw-popover-trigger"
            >
                {(popover_trigger.children)()}
            </div>
            <Follower slot show=is_show_popover placement>
                <div
                    class="thaw-popover"
                    style=move || css_vars.get()
                >
                    <div class=move || content_class.get()>{children()}</div>
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
