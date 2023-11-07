use super::use_breadcrumb_separator;
use leptos::*;

#[component]
pub fn BreadcrumbItem(children: Children) -> impl IntoView {
    let breadcrumb_separator = use_breadcrumb_separator();
    view! {
        <li class="thaw-breadcrumb-item">
            <span class="thaw-breadcrumb-item__link">
                {children()}
            </span>
            <span class="thaw-breadcrumb-item__separator">
                {move || breadcrumb_separator.0.get()}
            </span>
        </li>
    }
}
