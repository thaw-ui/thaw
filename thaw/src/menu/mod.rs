mod menu_item;

pub use menu_item::*;

use crate::ConfigInjection;
use leptos::{
    context::Provider,
    either::Either,
    ev::{self, on},
    html::Div,
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
    tachys::html::{class::class as tachys_class, node_ref::node_ref},
};
use std::time::Duration;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{class_list, mount_style, on_click_outside, ArcOneCallback, BoxOneCallback};

#[slot]
pub struct MenuTrigger<T> {
    children: TypedChildren<T>,
}

#[component]
pub fn Menu<T>(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The element or component that triggers menu.
    menu_trigger: MenuTrigger<T>,
    /// Action that displays the menu.
    #[prop(optional)]
    trigger_type: MenuTriggerType,
    /// Menu position.
    #[prop(optional)]
    position: MenuPosition,
    /// Called when item is selected.
    #[prop(into)]
    on_select: BoxOneCallback<String>,
    #[prop(optional, into)] appearance: MaybeProp<MenuAppearance>,
    children: Children,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    mount_style("menu", include_str!("./menu.css"));
    let config_provider = ConfigInjection::expect_context();

    let menu_ref = NodeRef::<Div>::new();
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

    let MenuTrigger {
        children: trigger_children,
    } = menu_trigger;
    let trigger_children = trigger_children.into_inner()()
        .into_inner()
        .add_any_attr(tachys_class(("thaw-menu-trigger", true)));

    let trigger_children = match trigger_type {
        MenuTriggerType::Click => {
            let trigger_ref = NodeRef::<thaw_utils::Element>::new();
            on_click_outside(
                move || {
                    if !is_show_menu.get_untracked() {
                        return None;
                    }
                    let Some(trigger_el) = trigger_ref.get_untracked() else {
                        return None;
                    };
                    let Some(menu_el) = menu_ref.get_untracked() else {
                        return None;
                    };
                    Some(vec![menu_el.into(), trigger_el])
                },
                move || is_show_menu.set(false),
            );
            Either::Left(
                trigger_children
                    .add_any_attr(node_ref(trigger_ref))
                    .add_any_attr(on(ev::click, move |_| {
                        is_show_menu.update(|show| {
                            *show = !*show;
                        });
                    })),
            )
        }
        MenuTriggerType::Hover => Either::Right(
            trigger_children
                .add_any_attr(on(ev::mouseenter, on_mouse_enter))
                .add_any_attr(on(ev::mouseleave, on_mouse_leave)),
        ),
    };

    let menu_injection = MenuInjection {
        has_icon: RwSignal::new(false),
        on_select: ArcOneCallback::new(move |value| {
            is_show_menu.set(false);
            on_select(value);
        }),
    };

    view! {
        <Binder>
            {trigger_children}
            <Follower slot show=is_show_menu placement=position>
                <Provider value=menu_injection>
                    <CSSTransition
                        name="menu-transition"
                        appear=is_show_menu.get_untracked()
                        show=is_show_menu
                        let:display
                    >
                        <div
                            class=class_list![
                                "thaw-config-provider thaw-menu",
                                move || appearance.get().map(|a| format!("thaw-menu--{}", a.as_str())),
                                class
                            ]
                            data-thaw-id=config_provider.id()
                            style=move || display.get().unwrap_or_default()
                            node_ref=menu_ref
                            on:mouseenter=on_mouse_enter
                            on:mouseleave=on_mouse_leave
                        >
                            {children()}
                        </div>
                    </CSSTransition>
                </Provider>
            </Follower>
        </Binder>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection {
    has_icon: RwSignal<bool>,
    on_select: ArcOneCallback<String>,
}

impl MenuInjection {
    pub fn expect_context() -> Self {
        expect_context()
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
pub enum MenuPosition {
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

impl From<MenuPosition> for FollowerPlacement {
    fn from(value: MenuPosition) -> Self {
        match value {
            MenuPosition::Top => Self::Top,
            MenuPosition::Bottom => Self::Bottom,
            MenuPosition::Left => Self::Left,
            MenuPosition::Right => Self::Right,
            MenuPosition::TopStart => Self::TopStart,
            MenuPosition::TopEnd => Self::TopEnd,
            MenuPosition::LeftStart => Self::LeftStart,
            MenuPosition::LeftEnd => Self::LeftEnd,
            MenuPosition::RightStart => Self::RightStart,
            MenuPosition::RightEnd => Self::RightEnd,
            MenuPosition::BottomStart => Self::BottomStart,
            MenuPosition::BottomEnd => Self::BottomEnd,
        }
    }
}
