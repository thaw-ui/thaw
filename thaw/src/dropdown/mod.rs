mod dropdown_item;
mod theme;

pub use dropdown_item::*;

use std::time::Duration;

use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
pub use theme::DropdownTheme;

use leptos::{html::Div, leptos_dom::helpers::TimeoutHandle, *};
use thaw_utils::{add_event_listener, mount_style, OptionalProp};

use crate::{use_theme, Theme};

#[slot]
pub struct DropdownTrigger {
    children: Children,
}

#[derive(Copy, Clone)]
struct HasIcon(RwSignal<bool>);

fn call_on_click_outside(element: NodeRef<Div>, on_click: Callback<()>) {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(ev::click, move |ev| {
            use leptos::wasm_bindgen::__rt::IntoJsResult;
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                let Some(dropdown_el) = element.get_untracked() else {
                    break;
                };
                if current_el == ***dropdown_el {
                    return;
                }
                el = current_el.parent_element();
            }
            on_click.call(());
        });
        on_cleanup(move || handle.remove());
    }
}

#[component]
pub fn Dropdown(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    dropdown_trigger: DropdownTrigger,
    #[prop(optional)] trigger_type: DropdownTriggerType,
    #[prop(optional)] placement: DropdownPlacement,
    children: Children,
) -> impl IntoView {
    mount_style("dropdown", include_str!("./dropdown.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
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
    let dropdown_ref = create_node_ref::<html::Div>();
    let target_ref = create_node_ref::<html::Div>();
    let is_show_dropdown = create_rw_signal(false);
    let show_dropdown_handle = store_value(None::<TimeoutHandle>);

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

    call_on_click_outside(
        dropdown_ref,
        Callback::new(move |_| is_show_dropdown.set(false)),
    );
    target_ref.on_load(move |target_el| {
        add_event_listener(target_el.into_any(), ev::click, move |event| {
            event.stop_propagation();
            is_show_dropdown.update(|show| *show = !*show);
        });
    });
    let DropdownTrigger {
        children: trigger_children,
    } = dropdown_trigger;

    provide_context(HasIcon(create_rw_signal(false)));

    view! {
        <Binder target_ref>
            <div
                class="thaw-dropdown-trigger"
                ref=target_ref
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

                        ref=dropdown_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        <div class=class.map(|c| move || c.get())>{children()}</div>
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
