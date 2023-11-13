mod tabbar_item;
mod theme;

use crate::{use_theme, utils::mount_style, Theme};
use leptos::*;
pub use tabbar_item::*;
pub use theme::TabbarTheme;

#[component]
pub fn Tabbar(
    #[prop(optional, into)] value: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tabbar", include_str!("./tabbar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--thaw-background-color: {};",
                theme.tabbar.background_color
            )
        })
    });
    provide_context(TabbarInjection(value));
    view! {
        <div class="thaw-tabbar" style=move || css_vars.get()>
            {children()}
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct TabbarInjection(pub RwSignal<String>);

pub(crate) fn use_tabbar() -> TabbarInjection {
    expect_context()
}
