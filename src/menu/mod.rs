mod menu_group;
mod menu_item;
mod theme;

use crate::utils::maybe_rw_signal::MaybeRwSignal;
use leptos::*;
pub use menu_group::MenuGroup;
pub use menu_item::*;
pub use theme::MenuTheme;

#[component]
pub fn Menu(
    #[prop(optional, into)] selected: MaybeRwSignal<String>,
    children: Children,
) -> impl IntoView {
    let menu_injection_key = create_rw_signal(MenuInjectionKey::new(selected.get_untracked()));
    create_effect(move |_| {
        let selected_key = selected.get();
        let key = menu_injection_key.get_untracked();
        if selected_key != key.value {
            menu_injection_key.set(MenuInjectionKey::new(selected_key));
        }
    });

    create_effect(move |_| {
        let selected_key = selected.get_untracked();
        let key = menu_injection_key.get();
        if selected_key != key.value {
            selected.set(key.value);
        }
    });
    provide_context(menu_injection_key);
    view! { <div class="melt-menu">{children()}</div> }
}

#[derive(Clone)]
pub struct MenuInjectionKey {
    value: String,
}

impl MenuInjectionKey {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

pub fn use_menu() -> RwSignal<MenuInjectionKey> {
    use_context::<RwSignal<MenuInjectionKey>>().expect("MenuInjectionKey not exist")
}
