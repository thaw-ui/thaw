mod menu_group;
mod menu_item;
mod theme;

use crate::utils::dyn_classes;
use crate::utils::ssr_class;

use leptos::*;
pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

#[component]
pub fn Menu(
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let ssr_class = ssr_class(&class);
    view! {
        <Provider value=MenuInjection(value)>
            <div class=ssr_class use:dyn_classes=class class="thaw-menu">
                {children()}
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection(pub RwSignal<String>);

pub(crate) fn use_menu() -> MenuInjection {
    expect_context()
}
