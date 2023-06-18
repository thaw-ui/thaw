mod tab;
use crate::{utils::mount_style::mount_style, theme::use_theme, Theme};
use leptos::*;
use stylers::style_sheet_str;
pub use tab::*;

#[component]
pub fn Tabs(cx: Scope, active_key: RwSignal<&'static str>, children: Children) -> impl IntoView {
    let class_name = mount_style("tabs", || style_sheet_str!("./src/tabs/tabs.css"));
    let tab_options_vec = create_rw_signal(cx, vec![]);
    provide_context(
        cx,
        TabsInjectionKey {
            active_key: active_key.clone(),
            tab_options_vec: tab_options_vec.clone(),
        },
    );
    let theme = use_theme(cx, Theme::light);
    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let color_primary = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--label-active-background-color: {color_primary};"));
        css_vars
    });
    view! { cx, class=class_name,
        <div class="melt-tabs" style=move || css_vars.get()>
            <div class="melt-tabs__label-list">
                <For each=move || tab_options_vec.get() key=move |v| v.key.clone() view=move |cx, options| view! {cx, class=class_name,
                    <span class="melt-tabs__label" class=("melt-tabs__label--active", move || options.key == active_key.get()) on:click=move |_| active_key.set(options.key)>
                        { options.label }
                    </span>
                }>
                </For>
                <span class="melt-tabs-label__active"></span>
            </div>
            <div>
                { children(cx) }
            </div>
        </div>
    }
}

#[derive(Clone)]
pub struct TabsInjectionKey {
    active_key: RwSignal<&'static str>,
    tab_options_vec: RwSignal<Vec<TabOptions>>,
}

impl TabsInjectionKey {
    pub fn get_key(&self) -> &str {
        self.active_key.get()
    }

    pub(crate) fn push_tab_options(&self, options: TabOptions) {
        self.tab_options_vec.update(|v| {
            v.push(options);
        });
    }
}

pub fn use_tabs(cx: Scope) -> TabsInjectionKey {
    use_context::<TabsInjectionKey>(cx).expect("TabsInjectionKey not exist")
}
