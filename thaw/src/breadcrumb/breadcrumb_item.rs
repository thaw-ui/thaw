use super::use_breadcrumb_separator;
use crate::utils::class_list::class_list;
use leptos::*;

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    let breadcrumb_separator = use_breadcrumb_separator();

    view! {
        <li class="thaw-breadcrumb-item">
            <span class=class_list![
                "thaw-breadcrumb-item__link", class.map(|c| move || c.get())
            ]>{children()}</span>
            <span class="thaw-breadcrumb-item__separator">
                {move || breadcrumb_separator.0.get()}
            </span>
        </li>
    }
}
