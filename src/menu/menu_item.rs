use super::{use_menu, MenuInjectionKey};
use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

#[component]
pub fn MenuItem(
    #[prop(into)] key: MaybeSignal<&'static str>,
    #[prop(into)] label: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));
    let theme = use_theme(Theme::light);
    let menu = use_menu();
    let onclick_select = move |_| {
        menu.set(MenuInjectionKey::new(key.get().to_string()));
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let font_color = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--font-color-active: {font_color};"));
        css_vars.push_str(&format!("--font-color: {};", theme.menu.color));
        css_vars.push_str(&format!("--bg-color: {font_color}1a;"));
        css_vars.push_str(&format!(
            "--bg-color-hover: {};",
            theme.menu.item_color_hover
        ));
        css_vars
    });
    view! {
        <div class="melt-menu-item">
            <div class="melt-menu-item__content" class=("melt-menu-item__content--selected", move || menu.get().value == key.get()) on:click=onclick_select style=move || css_vars.get()>
                { move || label.get() }
            </div>
        </div>
    }
}
