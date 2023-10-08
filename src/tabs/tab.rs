use super::use_tabs;
use crate::utils::mount_style::mount_style;
use leptos::*;

#[derive(Clone)]
pub(crate) struct TabOptions {
    pub key: &'static str,
    pub label: &'static str,
}

#[component]
pub fn Tab(key: &'static str, label: &'static str, children: Children) -> impl IntoView {
    mount_style("tab", include_str!("./tab.css"));
    let tabs = use_tabs();
    tabs.push_tab_options(TabOptions { key, label });
    view! {
        <div class="melt-tab" class=("melt-tab--hidden", move || key != tabs.get_key())>
            {children()}
        </div>
    }
}
