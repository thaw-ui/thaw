mod menu_group;
mod menu_item;
mod theme;

pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

use leptos::*;
use thaw_utils::{class_list, Model, OptionalProp};

#[component]
pub fn Menu(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    view! {
        <Provider value=MenuInjection(value)>
            <div class=class_list!["thaw-menu", class.map(| c | move || c.get())]>{children()}</div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct MenuInjection(pub Model<String>);

pub(crate) fn use_menu() -> MenuInjection {
    expect_context()
}
