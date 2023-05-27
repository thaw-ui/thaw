mod tabbar_item;

use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;
pub use tabbar_item::*;

#[component]
pub fn Tabbar(
    cx: Scope,
    #[prop(into)] selected: RwSignal<String>,
    children: Children,
) -> impl IntoView {
    let class_name = mount_style("tabbar", || {
        style_sheet_str!("./src/mobile/tabbar/tabbar.css")
    });

    let tabbar_injection_key = create_rw_signal(cx, TabbarInjectionKey::new(selected.get()));
    create_effect(cx, move |_| {
        let selected_key = selected.get();
        let key = tabbar_injection_key.get_untracked();
        if selected_key != key.value {
            tabbar_injection_key.set(TabbarInjectionKey::new(selected_key));
        }
    });

    create_effect(cx, move |_| {
        let selected_key = selected.get_untracked();
        let key = tabbar_injection_key.get();
        if selected_key != key.value {
            selected.set(key.value);
        }
    });
    provide_context(cx, tabbar_injection_key);
    view! {cx, class=class_name,
        <div class="melt-tabbar">
            { children(cx) }
        </div>
    }
}

#[derive(Clone)]
pub struct TabbarInjectionKey {
    value: String,
}

impl TabbarInjectionKey {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

pub fn use_tabbar(cx: Scope) -> RwSignal<TabbarInjectionKey> {
    use_context::<RwSignal<TabbarInjectionKey>>(cx).expect("TabbarInjectionKey not exist")
}
