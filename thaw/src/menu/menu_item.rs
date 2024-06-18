use super::use_menu;
use crate::{theme::use_theme, Icon, Theme};
use leptos::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal, OptionalProp};

#[component]
pub fn MenuItem(
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(optional, into)] icon: OptionalMaybeSignal<icondata_core::Icon>,
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));
    let theme = use_theme(Theme::light);
    let menu = use_menu();
    let click_key = key.clone();
    let on_click = move |_| {
        let click_key = click_key.get();
        if menu.0.with(|key| key != &click_key) {
            menu.0.set(click_key);
        }
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
                class=class_list![
                    "thaw-menu-item__content", ("thaw-menu-item__content--selected", move || menu.0
                    .get() == key.get()), class.map(| c | move || c.get())
                ]

                on:click=on_click
                style=move || css_vars.get()
            >
                {
                    move || {
                        view! {
                            <OptionComp value=icon.get() let:icon>
                                <Icon icon=icon style="font-size: 18px;margin-right: 8px"/>
                            </OptionComp>
                        }
                    }
                }
                {move || label.get()}
            </div>
        </div>
    }
}
