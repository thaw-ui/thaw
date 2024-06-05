mod tab;

pub use tab::*;

use leptos::*;
use std::collections::HashMap;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn TabList(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The value of the currently selected tab.
    #[prop(optional, into)]
    selected_value: Model<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tab-list", include_str!("./tab-list.css"));

    let registered_tabs = RwSignal::new(HashMap::new());

    view! {
        <Provider value=TabListInjection {
            previous_selected_value: StoredValue::new(selected_value.get_untracked()),
            selected_value,
            registered_tabs,
        }>
            <div
                class=class_list!["thaw-tab-list", class]
                role="tablist"
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct TabListInjection {
    pub previous_selected_value: StoredValue<String>,
    pub selected_value: Model<String>,
    pub registered_tabs: RwSignal<HashMap<String, TabRegisterData>>,
}

impl Copy for TabListInjection {}

impl TabListInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn register(&self, data: TabRegisterData) {
        self.registered_tabs.update(|map| {
            map.insert(data.value.clone(), data);
        });
    }

    pub fn unregister(&self, value: &String) {
        self.registered_tabs.update(|map| {
            map.remove(value);
        });
    }

    pub fn on_select(&self, value: String) {
        self.previous_selected_value
            .set_value(self.selected_value.get_untracked());
        self.selected_value.set(value);
    }
}

pub(crate) struct TabRegisterData {
    pub value: String,
    pub tab_ref: NodeRef<html::Button>,
}
