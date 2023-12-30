use leptos::*;

use crate::utils::{class_list::class_list, mount_style};

#[component]
pub fn Divider(#[prop(optional, into)] class: MaybeSignal<String>) -> impl IntoView {
    mount_style("divider", include_str!("./divider.css"));

    view! {
        <div class=class_list!["thaw-divider", move || class.get()]>
            <div class="thaw-divider__line"></div>
        </div>
    }
}
