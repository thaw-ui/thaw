use leptos::*;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::{mount_style, ssr_class};

#[component]
pub fn Divider(#[prop(optional, into)] class: MaybeSignal<String>) -> impl IntoView {
    mount_style("divider", include_str!("./divider.css"));
    let ssr_class = ssr_class(&class);
    view! {
        <div class="thaw-divider" class=ssr_class use:dyn_classes=class>
            <div class="thaw-divider__line"></div>
        </div>
    }
}

