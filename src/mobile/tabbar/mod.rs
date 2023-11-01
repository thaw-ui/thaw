mod tabbar_item;
mod theme;

use crate::{
    use_theme,
    utils::{maybe_rw_signal::MaybeRwSignal, mount_style::mount_style},
    Theme,
};
use leptos::*;
pub use tabbar_item::*;
pub use theme::TabbarTheme;

#[component]
pub fn Tabbar(
    #[prop(optional, into)] value: MaybeRwSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tabbar", include_str!("./tabbar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        theme.with(|theme| {
            format!(
                "--melt-background-color: {};",
                theme.tabbar.background_color
            )
        })
    });

    let tabbar_injection_key = create_rw_signal(TabbarInjectionKey::new(value.get()));
    create_effect(move |_| {
        let selected_key = value.get();
        let key = tabbar_injection_key.get_untracked();
        if selected_key != key.value {
            tabbar_injection_key.set(TabbarInjectionKey::new(selected_key));
        }
    });

    create_effect(move |_| {
        let selected_key = value.get_untracked();
        let key = tabbar_injection_key.get();
        if selected_key != key.value {
            value.set(key.value);
        }
    });
    provide_context(tabbar_injection_key);
    view! { <div class="melt-tabbar" style=move || css_vars.get()>{children()}</div> }
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

pub fn use_tabbar() -> RwSignal<TabbarInjectionKey> {
    use_context::<RwSignal<TabbarInjectionKey>>().expect("TabbarInjectionKey not exist")
}
