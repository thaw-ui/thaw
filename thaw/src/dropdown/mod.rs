mod dropdown_item;
mod theme;

pub use dropdown_item::*;

use std::time::Duration;

use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
pub use theme::DropdownTheme;

use leptos::{leptos_dom::helpers::TimeoutHandle, ev, html::Div, prelude::*};
use thaw_utils::{
    add_event_listener, call_on_click_outside, class_list, mount_style, OptionalProp,
};

use crate::{use_theme, Theme};

#[slot]
pub struct DropdownTrigger {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[derive(Copy, Clone)]
struct HasIcon(RwSignal<bool>);

#[derive(Copy, Clone)]
struct OnSelect(Callback<String>);

#[component]
pub fn Dropdown(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    dropdown_trigger: DropdownTrigger,
    #[prop(optional)] trigger_type: DropdownTriggerType,
    #[prop(optional)] placement: DropdownPlacement,
    #[prop(into)] on_select: Callback<String>,
    children: Children,
) -> impl IntoView {
    mount_style("dropdown", include_str!("./dropdown.css"));
    let theme = use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.dropdown.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.common.font_color));
        });
        css_vars
    });
    let dropdown_ref = NodeRef::<Div>::new();
    let target_ref = NodeRef::<Div>::new();
    let is_show_dropdown = RwSignal::new(false);
    let show_dropdown_handle = StoredValue::new(None::<TimeoutHandle>);

    let on_mouse_enter = move |_| {
        if trigger_type != DropdownTriggerType::Hover {
            return;
        }
        show_dropdown_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        is_show_dropdown.set(true);
    };
    let on_mouse_leave = move |_| {
        if trigger_type != DropdownTriggerType::Hover {
            return;
        }
        show_dropdown_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || {
                    is_show_dropdown.set(false);
                },
                Duration::from_millis(100),
            )
            .ok();
        });
    };

    if trigger_type != DropdownTriggerType::Hover {
        call_on_click_outside(
            dropdown_ref,
            Callback::new(move |_| is_show_dropdown.set(false)),
        );
    }

    Effect::new(move |_| {
        let Some(target_el) = target_ref.get() else {
            return;
        };
        add_event_listener(target_el.into(), ev::click, move |event| {
            if trigger_type != DropdownTriggerType::Click {
                return;
            }
            event.stop_propagation();
            is_show_dropdown.update(|show| *show = !*show);
        });
    });

    let DropdownTrigger {
        class: trigger_class,
        children: trigger_children,
    } = dropdown_trigger;

    provide_context(HasIcon(RwSignal::new(false)));
    provide_context(OnSelect(Callback::<String>::new(move |key| {
        is_show_dropdown.set(false);
        on_select.call(key);
    })));

    view! {
        <Binder target_ref>
            <div
                class=class_list!["thaw-dropdown-trigger", trigger_class.map(| c | move || c.get())]
                node_ref=target_ref
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {trigger_children()}
            </div>
            <Follower slot show=is_show_dropdown placement>
                <CSSTransition
                    node_ref=dropdown_ref
                    name="dropdown-transition"
                    appear=is_show_dropdown.get_untracked()
                    show=is_show_dropdown
                    let:display
                >
                    <div
                        class="thaw-dropdown"
                        style=move || {
                            display.get().map(|d| d.to_string()).unwrap_or_else(|| css_vars.get())
                        }

                        node_ref=dropdown_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        <div class=class_list![class.map(| c | move || c.get())]>{children()}</div>
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum DropdownTriggerType {
    Hover,
    #[default]
    Click,
}

impl Copy for DropdownTriggerType {}

#[derive(Default)]
pub enum DropdownPlacement {
    Top,
    #[default]
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

impl From<DropdownPlacement> for FollowerPlacement {
    fn from(value: DropdownPlacement) -> Self {
        match value {
            DropdownPlacement::Top => Self::Top,
            DropdownPlacement::Bottom => Self::Bottom,
            DropdownPlacement::Left => Self::Left,
            DropdownPlacement::Right => Self::Right,
            DropdownPlacement::TopStart => Self::TopStart,
            DropdownPlacement::TopEnd => Self::TopEnd,
            DropdownPlacement::LeftStart => Self::LeftStart,
            DropdownPlacement::LeftEnd => Self::LeftEnd,
            DropdownPlacement::RightStart => Self::RightStart,
            DropdownPlacement::RightEnd => Self::RightEnd,
            DropdownPlacement::BottomStart => Self::BottomStart,
            DropdownPlacement::BottomEnd => Self::BottomEnd,
        }
    }
}
