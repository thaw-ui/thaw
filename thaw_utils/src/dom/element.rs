use leptos::html::ElementType;
use send_wrapper::SendWrapper;
use std::ops::Deref;

#[derive(Debug, Clone)]
pub struct Element {
    el: SendWrapper<web_sys::Element>,
}

impl ElementType for Element {
    type Output = web_sys::Element;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Deref for Element {
    type Target = web_sys::Element;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}

#[derive(Debug, Clone)]
pub struct HtmlElement {
    el: SendWrapper<web_sys::HtmlElement>,
}

impl ElementType for HtmlElement {
    type Output = web_sys::HtmlElement;

    const TAG: &'static str = "";

    const SELF_CLOSING: bool = false;

    const ESCAPE_CHILDREN: bool = false;

    const NAMESPACE: Option<&'static str> = None;

    fn tag(&self) -> &str {
        ""
    }
}

impl Deref for HtmlElement {
    type Target = web_sys::HtmlElement;

    fn deref(&self) -> &Self::Target {
        &self.el
    }
}
