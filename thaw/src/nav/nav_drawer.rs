use crate::Scrollbar;
use leptos::{context::Provider, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, Model, OptionModel, OptionModelWithValue};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/nav/nav-drawer.css");

#[component]
pub fn NavDrawer(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The value of the currently selected navItem.
    #[prop(optional, into)]
    selected_value: OptionModel<String>,
    /// Indicates a category that has a selected child Will show the category as selected if it is closed.
    #[prop(optional, into)]
    selected_category_value: OptionModel<String>,
    /// Controls the open categories.
    #[prop(optional, into)]
    open_categories: Model<Vec<String>>,
    /// Indicates if NavDrawer supports multiple open Categories at the same time.
    #[prop(default = true.into(), into)]
    multiple: Signal<bool>,
    children: Children,
    #[prop(optional)] nav_drawer_header: Option<NavDrawerHeader>,
    #[prop(optional)] nav_drawer_footer: Option<NavDrawerFooter>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("nav-drawer", include_str!("./nav-drawer.css"));

    view! {
        <Provider value=NavDrawerInjection {
            selected_value,
            selected_category_value,
            open_categories,
            multiple,
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

#[derive(Clone, Copy)]
pub(crate) struct NavDrawerInjection {
    pub selected_value: OptionModel<String>,
    pub selected_category_value: OptionModel<String>,
    pub open_categories: Model<Vec<String>>,
    multiple: Signal<bool>,
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

    pub fn on_request_nav_category_item_toggle(&self, category_value: String) {
        self.open_categories.update(move |open_categories| {
            if self.multiple.get_untracked() {
                if let Some(index) = open_categories.iter().position(|v| v == &category_value) {
                    open_categories.remove(index);
                } else {
                    open_categories.push(category_value);
                }
            } else {
                if open_categories.first() == Some(&category_value) {
                    open_categories.clear();
                } else {
                    *open_categories = vec![category_value];
                }
            }
        });
    }
}
