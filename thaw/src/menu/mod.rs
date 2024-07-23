mod menu_item;

pub use menu_item::*;

use std::time::Duration;

use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};

use leptos::{leptos_dom::helpers::TimeoutHandle, ev, html::Div, prelude::*};
use thaw_utils::{
    add_event_listener, call_on_click_outside, class_list, mount_style, OptionalProp,
};

use crate::ConfigInjection;

#[slot]
pub struct MenuTrigger {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[derive(Copy, Clone)]
struct HasIcon(RwSignal<bool>);

#[derive(Copy, Clone)]
struct OnSelect(Callback<String>);

#[component]
pub fn Menu(
    #[prop(optional, into)] class: MaybeProp<String>,
    menu_trigger: MenuTrigger,
    #[prop(optional)] trigger_type: MenuTriggerType,
    #[prop(optional)] placement: MenuPlacement,
    #[prop(into)] on_select: Callback<String>,
    #[prop(optional, into)] appearance: Option<MaybeSignal<MenuAppearance>>,
    children: Children,
) -> impl IntoView {
    mount_style("menu", include_str!("./menu.css"));
    let config_provider = ConfigInjection::use_();

    let menu_ref = NodeRef::<Div>::new();
    let target_ref = NodeRef::<Div>::new();
    let is_show_menu = RwSignal::new(false);
    let show_menu_handle = StoredValue::new(None::<TimeoutHandle>);

    let on_mouse_enter = move |_| {
        if trigger_type != MenuTriggerType::Hover {
            return;
        }
        show_menu_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        is_show_menu.set(true);
    };
    let on_mouse_leave = move |_| {
        if trigger_type != MenuTriggerType::Hover {
            return;
        }
        show_menu_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || {
                    is_show_menu.set(false);
                },
                Duration::from_millis(100),
            )
            .ok();
        });
    };

    if trigger_type != MenuTriggerType::Hover {
        call_on_click_outside(
            menu_ref,
            Callback::new(move |_| is_show_menu.set(false)),
        );
    }

    Effect::new(move |_| {
        let Some(target_el) = target_ref.get() else {
            return;
        };
        let handler = add_event_listener(target_el.into(), ev::click, move |event| {
            if trigger_type != MenuTriggerType::Click {
                return;
            }
            event.stop_propagation();
            is_show_menu.update(|show| *show = !*show);
        });
        on_cleanup(move || handler.remove());
    });

    let MenuTrigger {
        class: trigger_class,
        children: trigger_children,
    } = menu_trigger;

    provide_context(HasIcon(RwSignal::new(false)));
    provide_context(OnSelect(Callback::<String>::new(move |key| {
        is_show_menu.set(false);
        on_select.call(key);
    })));

    view! {
        <Binder target_ref>
            <div
                class=class_list!["thaw-menu-trigger", trigger_class.map(| c | move || c.get())]
                node_ref=target_ref
                on:mouseenter=on_mouse_enter
                on:mouseleave=on_mouse_leave
            >
                {trigger_children()}
            </div>
            <Follower slot show=is_show_menu placement>
                <CSSTransition
                    node_ref=menu_ref
                    name="menu-transition"
                    appear=is_show_menu.get_untracked()
                    show=is_show_menu
                    let:display
                >
                    <div
                        class=class_list![
                            "thaw-config-provider thaw-menu",
                            appearance.map(|appearance| move || format!("thaw-menu--{}", appearance.get().as_str())),
                            class
                        ]
                        data-thaw-id=config_provider.id().clone()
                        style=move || display.get().unwrap_or_default()
                        node_ref=menu_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        {children()}
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Default, PartialEq, Clone)]
pub enum MenuTriggerType {
    Hover,
    #[default]
    Click,
}

impl Copy for MenuTriggerType {}

#[derive(Clone)]
pub enum MenuAppearance {
    Brand,
    Inverted,
}

impl MenuAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            MenuAppearance::Brand => "brand",
            MenuAppearance::Inverted => "inverted",
        }
    }
}

#[derive(Default)]
pub enum MenuPlacement {
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

impl From<MenuPlacement> for FollowerPlacement {
    fn from(value: MenuPlacement) -> Self {
        match value {
            MenuPlacement::Top => Self::Top,
            MenuPlacement::Bottom => Self::Bottom,
            MenuPlacement::Left => Self::Left,
            MenuPlacement::Right => Self::Right,
            MenuPlacement::TopStart => Self::TopStart,
            MenuPlacement::TopEnd => Self::TopEnd,
            MenuPlacement::LeftStart => Self::LeftStart,
            MenuPlacement::LeftEnd => Self::LeftEnd,
            MenuPlacement::RightStart => Self::RightStart,
            MenuPlacement::RightEnd => Self::RightEnd,
            MenuPlacement::BottomStart => Self::BottomStart,
            MenuPlacement::BottomEnd => Self::BottomEnd,
        }
    }
}
