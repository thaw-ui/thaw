mod menu_group;
mod menu_item;
mod theme;

use crate::utils::class_list::class_list;
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
    view! {
        <Provider value=MenuInjection(value)>
            <div class=class_list!["thaw-menu", move || class.get()]>{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection(pub RwSignal<String>);

pub(crate) fn use_menu() -> MenuInjection {
    expect_context()
}
