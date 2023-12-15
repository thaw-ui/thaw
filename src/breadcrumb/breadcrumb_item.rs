use super::use_breadcrumb_separator;
use leptos::*;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::ssr_class;

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let breadcrumb_separator = use_breadcrumb_separator();
    let ssr_class = ssr_class(&class);
    view! {
        <li class="thaw-breadcrumb-item">
            <span class=ssr_class use:dyn_classes=class class="thaw-breadcrumb-item__link">
                {children()}
            </span>
            <span class="thaw-breadcrumb-item__separator">
                {move || breadcrumb_separator.0.get()}
            </span>
        </li>
    }
}

