use leptos::*;
use thaw_utils::{mount_style, Model};

#[component]
pub fn NavDrawer(
    #[prop(optional, into)] selected_value: Model<String>,
    children: Children,
) -> impl IntoView {
    mount_style("nav-drawer", include_str!("./nav-drawer.css"));

    view! {
        <Provider value=NavDrawerInjection(selected_value)>
            <div class="thaw-nav-drawer">
                <div class="thaw-nav-drawer__body">
                    {children()}
                </div>
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct NavDrawerInjection(pub Model<String>);

pub(crate) fn use_nav_drawer() -> NavDrawerInjection {
    expect_context()
}
