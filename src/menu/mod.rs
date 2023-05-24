mod menu_item;

use leptos::*;
pub use menu_item::*;

#[component]
pub fn Menu(
    cx: Scope,
    #[prop(into)] selected: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let menu_injection_key = create_rw_signal(cx, MenuInjectionKey::new(selected.get()));
    create_effect(cx, move |_| {
        let selected_key = selected.get();
        let key = menu_injection_key.get_untracked();
        if selected_key != key.value {
            menu_injection_key.set(MenuInjectionKey::new(selected_key));
        }
    });

    create_effect(cx, move |_| {
        let selected_key = selected.get_untracked();
        let key = menu_injection_key.get();
        if selected_key != key.value {
            selected.set(key.value);
        }
    });
    provide_context(cx, menu_injection_key);
    view! {cx,
        <div class="melt-menu">
            { children(cx) }
        </div>
    }
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

pub fn use_menu(cx: Scope) -> RwSignal<MenuInjectionKey> {
    use_context::<RwSignal<MenuInjectionKey>>(cx).expect("MenuInjectionKey not exist")
}
