use super::use_tabs;
use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[derive(Clone)]
pub(crate) struct TabOptions {
    pub key: &'static str,
    pub label: &'static str,
}

#[component]
pub fn Tab(key: &'static str, label: &'static str, children: Children) -> impl IntoView {
    let class_name = mount_style("tab", || style_sheet_str!("./src/tabs/tab.css"));
    let tabs = use_tabs();
    tabs.push_tab_options(TabOptions { key, label });
    view! {  class=class_name,
        <div class="melt-tab" class=("melt-tab--hidden", move || key != tabs.get_key()) >
            { children() }
        </div>
    }
}
