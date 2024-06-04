use super::{TabListInjection, TabRegisterData};
use leptos::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Tab(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(into)] value: String,
    children: Children,
) -> impl IntoView {
    mount_style("tab", include_str!("./tab.css"));

    let tab_ref = NodeRef::<html::Button>::new();
    let tab_list = TabListInjection::use_();
    let value = StoredValue::new(value);
    tab_list.register(TabRegisterData {
        value: value.get_value(),
        tab_ref,
    });
    on_cleanup(move || {
        value.with_value(|v| tab_list.unregister(v));
    });

    let selected = Memo::new(move |_| {
        tab_list
            .selected_value
            .with(|selected_value| value.with_value(|value| value == selected_value))
    });

    view! {
        <button
            class=class_list![
                "thaw-tab", ("thaw-tab--hidden", move || selected.get()), class
            ]
            role="tab"
            aria-selected=move || if selected.get() { "true" } else { "false" }
            ref=tab_ref
        >
            <span class="thaw-tab__content">
                {children()}
            </span>
        </button>
    }
}
