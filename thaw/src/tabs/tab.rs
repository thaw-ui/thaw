use super::use_tabs;
use crate::utils::{class_list::class_list, mount_style, OptionalProp};
use leptos::*;

#[derive(Clone)]
pub(crate) struct TabOption {
    pub key: String,
    pub label: String,
    pub label_view: Option<TabLabelView>,
}

#[derive(Clone)]
pub(crate) struct TabLabelView {
    pub class: OptionalProp<MaybeSignal<String>>,
    pub children: Fragment,
}

impl From<TabLabel> for TabLabelView {
    fn from(tab_label: TabLabel) -> Self {
        let TabLabel { class, children } = tab_label;
        Self {
            class,
            children: children(),
        }
    }
}

#[slot]
pub struct TabLabel {
    #[prop(optional, into)]
    class: OptionalProp<MaybeSignal<String>>,
    children: Children,
}

#[component]
pub fn Tab(
    #[prop(into)] key: String,
    #[prop(optional, into)] label: String,
    #[prop(optional)] tab_label: Option<TabLabel>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("tab", include_str!("./tab.css"));
    let tabs = use_tabs();
    tabs.push_tab_options(TabOption {
        key: key.clone(),
        label,
        label_view: tab_label.map(|label| label.into()),
    });

    let is_active = create_memo({
        let key = key.clone();
        let tabs = tabs.clone();
        move |_| key == tabs.get_key()
    });

    on_cleanup(move || {
        tabs.remove_tab_options(&key);
    });

    view! {
        <div
            class=class_list![
                "thaw-tab", ("thaw-tab--hidden", move || ! is_active.get()), class.map(| c | move ||
                c.get())
            ]

            role="tabpanel"
            aria-hidden=move || if is_active.get() { "false" } else { "true" }
        >
            {children()}
        </div>
    }
}
