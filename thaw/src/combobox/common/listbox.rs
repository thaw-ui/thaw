use super::utils::{get_dropdown_action_from_key, DropdownAction};
use crate::{ConfigInjection, _aria::ActiveDescendantController};
use leptos::{context::Provider, ev, html, prelude::*};
use std::sync::Arc;
use thaw_components::CSSTransition;
use thaw_utils::{mount_style, BoxCallback};
use web_sys::{HtmlElement, Node};

#[component]
pub fn Listbox(
    open: ReadSignal<bool>,
    class: &'static str,
    set_listbox: Arc<dyn Fn(Node) + Send + Sync>,
    listbox_ref: NodeRef<html::Div>,
    #[prop(optional)] on_hidden: StoredValue<Vec<BoxCallback>>,
    children: Children,
) -> impl IntoView {
    mount_style("listbox", include_str!("./listbox.css"));

    let config_provider = ConfigInjection::expect_context();
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
    let on_after_leave = move || {
        if let Some(list) =
            on_hidden.try_update_value(|list| list.drain(..).collect::<Vec<BoxCallback>>())
        {
            list.into_iter().for_each(|f| f());
        }
    };
    on_cleanup(move || {
        drop(effect);
    });

    view! {
        <Provider value=ListboxInjection(trigger)>
            <CSSTransition
                name="fade-in-scale-up-transition"
                appear=open.get_untracked()
                show=open
                let:display
                on_after_leave
            >
                <div
                    class=format!("thaw-config-provider thaw-listbox {class}")
                    style=move || display.get().unwrap_or_default()
                    data-thaw-id=config_provider.id()
                    node_ref=listbox_ref
                    role="listbox"
                    on:mousedown=|e| e.prevent_default()
                >
                    {children()}
                </div>
            </CSSTransition>
        </Provider>
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
