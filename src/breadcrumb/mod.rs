mod breadcrumb_item;

pub use breadcrumb_item::BreadcrumbItem;
use leptos::*;

use crate::mount_style;

#[component]
pub fn Breadcrumb(
    #[prop(default = MaybeSignal::Static("/".to_string()),into)] separator: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("breadcrumb", include_str!("./breadcrumb.css"));
    provide_context(BreadcrumbSeparatorInjection(separator));
    view! {
        <nav class="thaw-breadcrumb">
            <ul>
                {children()}
            </ul>
        </nav>
    }
}

#[derive(Clone)]
pub(crate) struct BreadcrumbSeparatorInjection(MaybeSignal<String>);

pub(crate) fn use_breadcrumb_separator() -> BreadcrumbSeparatorInjection {
    expect_context()
}
