use super::{use_tabbar, TabbarInjectionKey};
use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
use stylers::style_sheet_str;
use crate::components::*;
use leptos_icons::*;

#[component]
pub fn TabbarItem(
    cx: Scope,
    #[prop(into)] name: MaybeSignal<&'static str>,
    #[prop(optional, into)] icon: Option<Icon>,
    children: Children,
) -> impl IntoView {
    let class_name = mount_style("tabbar-item", || style_sheet_str!("./src/mobile/tabbar/tabbar-item.css"));
    let theme = use_theme(cx, Theme::light);
    let tabbar = use_tabbar(cx);
    let onclick_select = move |_| {
        tabbar.set(TabbarInjectionKey::new(name.get().to_string()));
    };

    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let font_color = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--font-color-selected: {font_color};"));
        css_vars
    });

    view! {cx, class=class_name,
        <div class="melt-tabbar-item" class=("melt-tabbar-item--selected", move || tabbar.get().value == name.get()) on:click=onclick_select style=move || css_vars.get()>
            <OptionComp value=icon bind:icon>
                <Icon icon=icon width="22px" height="22px" class="melt-tabbar-item__icon"/>
            </OptionComp>
            <div class="melt-tabbar-item__content">
                { children(cx) }
            </div>
        </div>
    }
}
