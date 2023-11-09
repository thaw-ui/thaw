mod menu_group;
mod menu_item;
mod theme;

use leptos::*;
pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

#[component]
pub fn Menu(#[prop(optional, into)] value: RwSignal<String>, children: Children) -> impl IntoView {
    provide_context(MenuInjection(value));
    view! { <div class="thaw-menu">{children()}</div> }
}

#[derive(Clone)]
pub(crate) struct MenuInjection(pub RwSignal<String>);

pub(crate) fn use_menu() -> MenuInjection {
    expect_context()
}
