mod menu_group;
mod menu_item;
mod theme;

pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

use leptos::*;
use std::collections::BTreeSet;
use thaw_utils::{class_list, Model, OptionalProp};

#[component]
pub fn Menu(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] default_expanded_keys: Vec<String>,
    children: Children,
) -> impl IntoView {
    let path = RwSignal::new(BTreeSet::<String>::new());

    view! {
        <Provider value=MenuInjection { value, path, default_expanded_keys: StoredValue::new(default_expanded_keys) }>
            <div class=class_list!["thaw-menu", class.map(| c | move || c.get())]>{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection {
    pub value: Model<String>,
    pub path: RwSignal<BTreeSet<String>>,
    pub default_expanded_keys: StoredValue<Vec<String>>,
}

impl MenuInjection {
    pub fn use_() -> Self {
        expect_context()
    }
}
