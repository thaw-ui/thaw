use super::use_tabbar;
use crate::components::*;
use crate::utils::StoredMaybeSignal;
use crate::{icon::*, theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

#[component]
pub fn TabbarItem(
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(optional, into)] icon: Option<Icon>,
    children: Children,
) -> impl IntoView {
    mount_style("tabbar-item", include_str!("./tabbar-item.css"));
    let theme = use_theme(Theme::light);
    let tabbar = use_tabbar();
    let key: StoredMaybeSignal<_> = key.into();
    let on_click = move |_| {
        let click_key = key.get();
        if tabbar.0.with(|key| key != &click_key) {
            tabbar.0.set(click_key);
        }
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let font_color = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-font-color-selected: {font_color};"));
        });
        css_vars
    });

    view! {
        <div
            class="thaw-tabbar-item"
            class=("thaw-tabbar-item--selected", move || tabbar.0.get() == key.get())
            on:click=on_click
            style=move || css_vars.get()
        >
            <OptionComp value=icon let:icon>
                <Icon icon=icon width="22px" height="22px" class="thaw-tabbar-item__icon"/>
            </OptionComp>
            <div class="thaw-tabbar-item__content">{children()}</div>
        </div>
    }
}
