use super::use_tabs;
#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::{mount_style, ssr_class};
use leptos::*;

#[derive(Clone)]
pub(crate) struct TabOption {
    pub key: String,
    pub label: String,
}

#[component]
pub fn Tab(
    #[prop(into)] key: String,
    #[prop(into)] label: String,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tab", include_str!("./tab.css"));
    let tabs = use_tabs();
    tabs.push_tab_options(TabOption {
        key: key.clone(),
        label,
    });
    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class="thaw-tab"
            class=("thaw-tab--hidden", move || key != tabs.get_key())
        >
            {children()}
        </div>
    }
}
