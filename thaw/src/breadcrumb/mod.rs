use leptos::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Breadcrumb(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("breadcrumb", include_str!("./breadcrumb.css"));

    view! {
        <nav
            class=class_list!["thaw-breadcrumb", class]
        >
            <ol role="list" class="thaw-breadcrumb__list">{children()}</ol>
        </nav>
    }
}

#[component]
pub fn BreadcrumbItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <li class=class_list!["thaw-breadcrumb-item", class]>
            {children()}
        </li>
    }
}

#[component]
pub fn BreadcrumbButton(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] current: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <button
            class=class_list!["thaw-breadcrumb-button", ("thaw-breadcrumb-button--current", move || current.get()), class]
            aria-disabled=move || current.get().then(|| "true")
            aria-current=move || current.get().then(|| "page")
        >
            {children()}
        </button>
    }
}

#[component]
pub fn BreadcrumbDivider(#[prop(optional, into)] class: MaybeProp<String>) -> impl IntoView {
    view! {
        <li class=class_list!["thaw-breadcrumb-divider", class] aria-hidden="true">
            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                <path d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z" fill="currentColor"></path>
            </svg>
        </li>
    }
}
