use crate::Scrollbar;
use leptos::*;
use thaw_components::OptionComp;
use thaw_utils::{mount_style, Model};

#[component]
pub fn NavDrawer(
    #[prop(optional, into)] selected_value: Model<String>,
    children: Children,
    #[prop(optional)] nav_drawer_header: Option<NavDrawerHeader>,
    #[prop(optional)] nav_drawer_footer: Option<NavDrawerFooter>,
) -> impl IntoView {
    mount_style("nav-drawer", include_str!("./nav-drawer.css"));

    view! {
        <Provider value=NavDrawerInjection(selected_value)>
            <div class="thaw-nav-drawer">
                <OptionComp value=nav_drawer_header let:header>
                    <header class="thaw-nav-drawer__header">{(header.children)()}</header>
                </OptionComp>
                <div class="thaw-nav-drawer__body">
                    <Scrollbar>
                        {children()}
                    </Scrollbar>
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
pub(crate) struct NavDrawerInjection(pub Model<String>);

pub(crate) fn use_nav_drawer() -> NavDrawerInjection {
    expect_context()
}
