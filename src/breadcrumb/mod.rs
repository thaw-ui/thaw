mod breadcrumb_item;
mod theme;

use crate::{
    use_theme,
    utils::{mount_style, Provider},
    Theme,
};
pub use breadcrumb_item::BreadcrumbItem;
use leptos::*;
pub use theme::BreadcrumbTheme;

#[component]
pub fn Breadcrumb(
    #[prop(default = MaybeSignal::Static("/".to_string()),into)] separator: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("breadcrumb", include_str!("./breadcrumb.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-font-color: {};",
                theme.breadcrumb.item_font_color
            ));
            css_vars.push_str(&format!(
                "--thaw-font-color-hover: {};",
                theme.breadcrumb.item_font_color_hover
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.breadcrumb.item_background_color_hover
            ));
        });
        css_vars
    });
    view! {
        <Provider value=BreadcrumbSeparatorInjection(separator)>
            <nav class="thaw-breadcrumb" style=move || css_vars.get()>
                <ul>{children()}</ul>
            </nav>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct BreadcrumbSeparatorInjection(MaybeSignal<String>);

pub(crate) fn use_breadcrumb_separator() -> BreadcrumbSeparatorInjection {
    expect_context()
}
