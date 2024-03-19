mod tabbar_item;
mod theme;

pub use tabbar_item::*;
pub use theme::TabbarTheme;

use crate::{use_theme, Theme};
use leptos::*;
use thaw_utils::{mount_style, Model};

#[component]
pub fn Tabbar(#[prop(optional, into)] value: Model<String>, children: Children) -> impl IntoView {
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

    view! {
        <Provider value=TabbarInjection(value)>
            <div class="thaw-tabbar" style=move || css_vars.get()>
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct TabbarInjection(pub Model<String>);

pub(crate) fn use_tabbar() -> TabbarInjection {
    expect_context()
}
