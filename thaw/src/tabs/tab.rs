use super::use_tabs;
use crate::utils::{class_list::class_list, mount_style};
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

    on_cleanup({
        let key = key.clone();
        let tabs = tabs.clone();
        move || {
            tabs.remove_tab_options(&key);
        }
    });

    view! {
        <div class=class_list![
            "thaw-tab", ("thaw-tab--hidden", move || key != tabs.get_key()), move || class.get()
        ]>{children()}</div>
    }
}
