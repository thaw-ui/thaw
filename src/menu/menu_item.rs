use super::{use_menu, MenuInjectionKey};
use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

#[component]
pub fn MenuItem(
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(into)] label: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));
    let theme = use_theme(Theme::light);
    let menu = use_menu();
    let click_key = key.clone();
    let on_click = move |_| {
        menu.set(MenuInjectionKey::new(click_key.get()));
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let font_color = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-font-color-active: {font_color};"));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.menu.color));
            css_vars.push_str(&format!("--thaw-background-color: {font_color}1a;"));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.menu.item_color_hover
            ));
        });
        css_vars
    });
    view! {
        <div class="thaw-menu-item">
            <div
                class="thaw-menu-item__content"
                class=("thaw-menu-item__content--selected", move || menu.get().value == key.get())
                on:click=on_click
                style=move || css_vars.get()
            >
                {move || label.get()}
            </div>
        </div>
    }
}
