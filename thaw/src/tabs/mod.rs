mod tab;

pub use tab::*;

use leptos::*;
use std::collections::HashMap;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn TabList(
    #[prop(optional, into)] selected_value: Model<String>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tab-list", include_str!("./tab-list.css"));

    let registered_tabs = RwSignal::new(HashMap::new());
    // request_animation_frame(move || {
    //     let list_rect = label_list.get_bounding_client_rect();
    //     let rect = label.get_bounding_client_rect();
    //     label_line
    //         .set(
    //             Some(TabsLabelLine {
    //                 width: rect.width(),
    //                 left: rect.left() - list_rect.left(),
    //             }),
    //         );
    // });
    view! {
        <Provider value=TabListInjection {
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
    pub selected_value: Model<String>,
    registered_tabs: RwSignal<HashMap<String, TabRegisterData>>,
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
}

pub(crate) struct TabRegisterData {
    value: String,
    tab_ref: NodeRef<html::Button>,
}
