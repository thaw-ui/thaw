use super::use_tabster::use_tabster;
use tabster::types::FindFirstProps;
use web_sys::HtmlElement;

pub struct FocusFinders {
    pub find_last_focusable: Box<dyn FnMut(HtmlElement) -> Option<HtmlElement>>,
}

pub fn use_focus_finders() -> FocusFinders {
    let tabster = use_tabster();

    let find_last_focusable = move |container| {
        let mut focusable = tabster.focusable.borrow_mut();
        focusable.find_last(FindFirstProps { container }, Default::default())
    };

    FocusFinders {
        find_last_focusable: Box::new(find_last_focusable),
    }
}
