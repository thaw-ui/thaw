use crate::Scrollbar;
use leptos::{context::Provider, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, OptionModel, OptionModelWithValue};

#[component]
pub fn NavDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The value of the currently selected navItem.
    #[prop(optional, into)]
    selected_value: OptionModel<String>,
    /// Indicates a category that has a selected child Will show the category as selected if it is closed.
    #[prop(optional, into)]
    selected_category_value: OptionModel<String>,
    children: Children,
    #[prop(optional)] nav_drawer_header: Option<NavDrawerHeader>,
    #[prop(optional)] nav_drawer_footer: Option<NavDrawerFooter>,
) -> impl IntoView {
    mount_style("nav-drawer", include_str!("./nav-drawer.css"));

    view! {
        <Provider value=NavDrawerInjection {
            selected_value,
            selected_category_value,
        }>
            <div class=class_list!["thaw-nav-drawer", class]>
                <OptionComp value=nav_drawer_header let:header>
                    <header class="thaw-nav-drawer__header">{(header.children)()}</header>
                </OptionComp>
                <div class="thaw-nav-drawer__body">
                    <Scrollbar>{children()}</Scrollbar>
                </div>
                <OptionComp value=nav_drawer_footer let:footer>
                    <footer class="thaw-nav-drawer__footer">{(footer.children)()}</footer>
                </OptionComp>
            </div>
        </Provider>
    }
}

#[slot]
pub struct NavDrawerHeader {
    children: Children,
}

#[slot]
pub struct NavDrawerFooter {
    children: Children,
}

#[derive(Clone)]
pub(crate) struct NavDrawerInjection {
    pub selected_value: OptionModel<String>,
    pub selected_category_value: OptionModel<String>,
}

impl NavDrawerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn is_selected_category(&self, value: &String) -> bool {
        self.selected_category_value
            .with(|selected_category_value| match selected_category_value {
                OptionModelWithValue::T(v) => v == value,
                OptionModelWithValue::Option(v) => v.as_ref() == Some(value),
            })
    }
}
