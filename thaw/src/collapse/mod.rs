mod collapse_item;
mod theme;

pub use collapse_item::CollapseItem;
pub use theme::CollapseTheme;

use crate::{use_theme, Theme};
use leptos::*;
use std::collections::HashSet;
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Collapse(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] value: Model<HashSet<String>>,
    #[prop(optional)] accordion: bool,
    children: Children,
) -> impl IntoView {
    mount_style("collapser", include_str!("./collapse.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| format!("--thaw-border-color: {};", theme.collapse.border_color))
    });

    view! {
        <Provider value=CollapseInjection {
            value,
            accordion,
        }>
            <div
                class=class_list!["thaw-collapse", class.map(| c | move || c.get())]
                style=move || css_vars.get()
            >
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct CollapseInjection {
    pub value: Model<HashSet<String>>,
    pub accordion: bool,
}

pub(crate) fn use_collapse() -> CollapseInjection {
    expect_context()
}
