mod accordion_item;

pub use accordion_item::*;

use leptos::{context::Provider, prelude::*};
use std::collections::HashSet;
use thaw_utils::{class_list, Model};

#[component]
pub fn Accordion(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Controls the state of the panel.
    #[prop(optional, into)]
    open_items: Model<HashSet<String>>,
    /// Indicates if Accordion support multiple Panels opened at the same time.
    #[prop(optional)]
    multiple: bool,
    /// Indicates if Accordion support multiple Panels closed at the same time.
    #[prop(optional)]
    collapsible: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=AccordionInjection {
            open_items,
            collapsible,
            multiple,
        }>
            <div class=class_list!["thaw-accordion", class]>{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct AccordionInjection {
    pub open_items: Model<HashSet<String>>,
    pub multiple: bool,
    pub collapsible: bool,
}

impl AccordionInjection {
    pub fn expect_context() -> AccordionInjection {
        expect_context()
    }
}
