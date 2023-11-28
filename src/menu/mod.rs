mod menu_group;
mod menu_item;
mod theme;

use leptos::*;
pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

#[component]
pub fn Menu(#[prop(optional, into)] value: RwSignal<String>, children: Children) -> impl IntoView {
    view! {
        <Provider value=MenuInjection(value)>
            <div class="thaw-menu">{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection(pub RwSignal<String>);

pub(crate) fn use_menu() -> MenuInjection {
    expect_context()
}
