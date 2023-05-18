mod menu_item;

use leptos::*;
pub use menu_item::*;

#[component]
pub fn Menu(
    cx: Scope,
    #[prop(into)] selected: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let menu_injection_key = create_rw_signal(cx, MenuInjectionKey::new(selected));
    provide_context(cx, menu_injection_key);
    view! {cx,
        <div class="melt-menu">
            { children(cx) }
        </div>
    }
}

#[derive(Clone)]
pub struct MenuInjectionKey {
    value: RwSignal<String>,
}


impl MenuInjectionKey {
    pub fn new(value: RwSignal<String>) -> Self {
        Self { value }
    }

    pub fn from_string(cx: Scope, value: String) -> Self {
        Self {
            value: create_rw_signal(cx, value),
        }
    }

    pub fn value(&self) -> String {
        self.value.get()
    }
}

pub fn use_menu(cx: Scope) -> RwSignal<MenuInjectionKey> {
    use_context::<RwSignal<MenuInjectionKey>>(cx).expect("MenuInjectionKey not exist")
}
