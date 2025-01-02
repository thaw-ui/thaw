use super::utils::{get_dropdown_action_from_key, DropdownAction};
use crate::_aria::ActiveDescendantController;
use leptos::{context::Provider, ev, html, prelude::*};
use std::sync::Arc;
use thaw_utils::mount_style;
use web_sys::{HtmlElement, Node};

#[component]
pub fn Listbox(
    class: &'static str,
    set_listbox: Arc<dyn Fn(Node) + Send + Sync>,
    listbox_ref: NodeRef<html::Div>,
    children: Children,
) -> impl IntoView {
    mount_style("listbox", include_str!("./listbox.css"));

    let trigger = ArcTrigger::new();
    let effect = RenderEffect::new({
        let trigger = trigger.clone();
        move |_| {
            trigger.track();
            if let Some(listbox_el) = listbox_ref.get() {
                set_listbox(listbox_el.into());
            }
        }
    });

    on_cleanup(move || {
        drop(effect);
    });

    view! {
        <div
            class=format!("thaw-listbox {class}")
            node_ref=listbox_ref
            role="listbox"
            on:mousedown=|e| e.prevent_default()
        >
            <Provider value=ListboxInjection(trigger)>{children()}</Provider>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct ListboxInjection(ArcTrigger);

impl ListboxInjection {
    #[inline]
    pub fn expect_context() -> Self {
        expect_context()
    }

    #[inline]
    pub fn trigger(&self) {
        self.0.notify();
    }
}

pub fn listbox_keyboard_event(
    e: ev::KeyboardEvent,
    open: RwSignal<bool>,
    multiselect: bool,
    active_descendant_controller: &ActiveDescendantController,
    select_option: impl Fn(HtmlElement),
) {
    let (open, set_open) = open.split();
    let open = open.get_untracked();
    let action = get_dropdown_action_from_key(&e, open, multiselect);
    let active_option = active_descendant_controller.active();

    match action {
        DropdownAction::Type | DropdownAction::Open => {
            if !open {
                set_open.set(true);
            }
            if action == DropdownAction::Open {
                e.prevent_default();
            }
        }
        DropdownAction::CloseSelect | DropdownAction::Select => {
            e.prevent_default();
            if let Some(option) = active_option {
                select_option(option);
            }
        }
        DropdownAction::Next => {
            e.prevent_default();
            if active_option.is_some() {
                active_descendant_controller.next();
            } else {
                active_descendant_controller.first();
            }
        }
        DropdownAction::Previous => {
            e.prevent_default();
            if active_option.is_some() {
                active_descendant_controller.prev();
            } else {
                active_descendant_controller.first();
            }
        }
        DropdownAction::First | DropdownAction::PageUp => {
            e.prevent_default();
            active_descendant_controller.first();
        }
        DropdownAction::Last | DropdownAction::PageDown => {
            e.prevent_default();
            active_descendant_controller.last();
        }
        DropdownAction::Tab | DropdownAction::Close | DropdownAction::None => {}
    };
}
