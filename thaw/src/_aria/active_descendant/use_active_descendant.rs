use super::use_option_walker::{use_option_walker, OptionWalker};
use send_wrapper::SendWrapper;
use std::{cell::RefCell, sync::Arc};
use thaw_utils::scroll_into_view;
use web_sys::{HtmlElement, Node};

/// Applied to the element that is active descendant
const ACTIVEDESCENDANT_ATTRIBUTE: &str = "data-activedescendant";

/// Applied to the active descendant when the user is navigating with keyboard
const ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE: &str = "data-activedescendant-focusvisible";

pub fn use_active_descendant<MF>(
    match_option: MF,
) -> (Arc<dyn Fn(Node) + Send + Sync>, ActiveDescendantController)
where
    MF: Fn(HtmlElement) -> bool + Send + Sync + 'static,
{
    let (set_listbox, option_walker) = use_option_walker(match_option);
    //TODO
    let set_listbox = Arc::new(move |node| {
        set_listbox(&node);
    });
    let controller = ActiveDescendantController {
        option_walker,
        active: Arc::new(SendWrapper::new(Default::default())),
        // last_active: Default::default(),
    };

    (set_listbox, controller)
}

#[derive(Clone)]
pub struct ActiveDescendantController {
    option_walker: OptionWalker,
    active: Arc<SendWrapper<RefCell<Option<HtmlElement>>>>,
    // last_active: RefCell<Option<HtmlElement>>,
}

impl ActiveDescendantController {
    fn blur_active_descendant(&self) {
        let mut active = self.active.borrow_mut();
        let Some(active_el) = active.as_mut() else {
            return;
        };
        let _ = active_el.remove_attribute(ACTIVEDESCENDANT_ATTRIBUTE);
        let _ = active_el.remove_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE);

        *active = None;
    }

    fn focus_active_descendant(&self, next_active: HtmlElement) {
        self.blur_active_descendant();
        scroll_into_view(&next_active);
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_ATTRIBUTE, "");
        let _ = next_active.set_attribute(ACTIVEDESCENDANT_FOCUSVISIBLE_ATTRIBUTE, "");

        *self.active.borrow_mut() = Some(next_active);
    }
}

impl ActiveDescendantController {
    pub fn first(&self) {
        if let Some(first) = self.option_walker.first() {
            self.focus_active_descendant(first);
        }
    }

    pub fn last(&self) {
        if let Some(last) = self.option_walker.last() {
            self.focus_active_descendant(last);
        }
    }

    pub fn next(&self) {
        if let Some(next) = self.option_walker.next() {
            self.focus_active_descendant(next);
        }
    }

    pub fn prev(&self) {
        if let Some(prev) = self.option_walker.prev() {
            self.focus_active_descendant(prev);
        }
    }

    pub fn blur(&self) {
        self.blur_active_descendant();
    }

    pub fn active(&self) -> Option<HtmlElement> {
        let active = self.active.borrow();
        if let Some(active) = active.as_ref() {
            Some(active.clone())
        } else {
            None
        }
    }
}
