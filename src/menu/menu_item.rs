use super::{use_menu, MenuInjectionKey};
use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn MenuItem(
    cx: Scope,
    #[prop(into)] key: MaybeSignal<&'static str>,
    #[prop(into)] label: MaybeSignal<String>,
) -> impl IntoView {
    let class_name = mount_style("menu-item", || style_sheet_str!("./src/menu/menu-item.css"));
    let theme = use_theme(cx, Theme::light);
    let menu = use_menu(cx);
    let onclick_select = move |_| {
        menu.set(MenuInjectionKey::from_string(cx, key.get().to_string()));
    };

    let css_vars = create_memo(cx, move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let font_color = theme.common.color_primary.clone();
        css_vars.push_str(&format!("--font-color: {font_color};"));
        css_vars.push_str(&format!("--bg-color: {font_color}1a;"));
        let border_radius = theme.common.border_radius.clone();
        css_vars.push_str(&format!("--border-radius: {border_radius};"));
        css_vars
    });
    view! {cx, class=class_name,
        <div class="melt-menu-item">
            <div class="melt-menu-item__content" class=("melt-menu-item__content--selected", move || menu.get().value() == key.get()) on:click=onclick_select style=move || css_vars.get()>
                { move || label.get() }
            </div>
        </div>
    }
}
