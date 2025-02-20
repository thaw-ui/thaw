use super::constants::FOCUS_VISIBLE_ATTR;
use gloo_events::EventListener;
use keyborg::{Keyborg, KEYBORG_FOCUSIN};
use send_wrapper::SendWrapper;
use std::sync::{Arc, RwLock};
use web_sys::{
    wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt},
    Element, Event, HtmlElement, Window,
};

pub fn apply_focus_visible_polyfill(
    scope: HtmlElement,
    target_window: Window,
) -> Box<dyn FnOnce() + Send + Sync + 'static> {
    if already_in_scope(Some(&scope)) {
        // Focus visible polyfill already applied at this scope
        return Box::new(move || {});
    }

    let state = Arc::new(RwLock::new(None::<SendWrapper<HtmlElement>>));
    let keyborg = Keyborg::create(target_window.clone(), None);

    let register_element_if_navigating = {
        let keyborg = keyborg.clone();
        let state = state.clone();
        move |el: Option<JsValue>| {
            if keyborg.read().unwrap_throw().is_navigating_with_keyboard() {
                if let Some(el) = el {
                    if let Ok(el) = el.dyn_into::<HtmlElement>() {
                        *state.write().unwrap_throw() = Some(SendWrapper::new(el.clone()));
                        let _ = el.set_attribute(FOCUS_VISIBLE_ATTR, "");
                    }
                }
            }
        }
    };

    let dispose_current_element = move || {
        if let Some(el) = state.read().unwrap_throw().as_ref() {
            let _ = el.remove_attribute(FOCUS_VISIBLE_ATTR);
        }
        *state.write().unwrap_throw() = None;
    };

    keyborg.write().unwrap_throw().subscribe({
        let dispose_current_element = dispose_current_element.clone();
        move |is_navigating_with_keyboard| {
            // When navigation mode changes remove the focus-visible selector
            if !is_navigating_with_keyboard {
                dispose_current_element();
            }
        }
    });

    // Keyborg's focusin event is delegated so it's only registered once on the window
    // and contains metadata about the focus event
    let keyborg_listener = {
        let register_element_if_navigating = register_element_if_navigating.clone();
        let dispose_current_element = dispose_current_element.clone();
        move |e: &Event| {
            dispose_current_element();
            let target = e.composed_path().at(0);
            register_element_if_navigating(Some(target));
        }
    };

    // Make sure that when focus leaves the scope, the focus visible class is removed
    let blur_listener = {
        let dispose_current_element = dispose_current_element.clone();
        let scope = scope.clone();
        move |event: &Event| {
            let e = event.dyn_ref::<web_sys::FocusEvent>().unwrap_throw();
            let Some(related_target) = e.related_target() else {
                dispose_current_element();
                return;
            };
            let Some(el) = related_target.dyn_ref::<HtmlElement>() else {
                return;
            };

            if !scope.contains(Some(el)) {
                dispose_current_element();
            }
        }
    };

    let keyborg_listener = EventListener::new(&scope, KEYBORG_FOCUSIN, keyborg_listener);
    let blur_listener = EventListener::new(&scope, "focusout", blur_listener);
    // (scope as HTMLElementWithFocusVisibleScope).focusVisible = true;

    let doc = target_window.document().unwrap_throw();
    if scope.contains(doc.active_element().as_deref()) {
        register_element_if_navigating(doc.active_element().map(|el| el.into()));
    }

    Box::new(move || {
        let _ = keyborg_listener;
        let _ = blur_listener;
        // delete (scope as HTMLElementWithFocusVisibleScope).focusVisible;
        keyborg.read().unwrap_throw().dispose();
    })
}

fn already_in_scope(el: Option<&Element>) -> bool {
    let Some(el) = el else {
        return false;
    };

    // if ((el as HTMLElementWithFocusVisibleScope).focusVisible) {
    //   return true;
    // }

    already_in_scope(el.parent_element().as_ref())
}
