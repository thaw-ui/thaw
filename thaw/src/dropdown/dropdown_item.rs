use leptos::*;
use thaw_components::{Fallback, If, OptionComp, Then};
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal, OptionalProp};

use crate::{dropdown::HasIcon, use_theme, Icon, Theme};

#[component]
pub fn DropdownItem(
    #[prop(optional, into)] icon: OptionalMaybeSignal<icondata_core::Icon>,
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    mount_style("dropdown-item", include_str!("./dropdown-item.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.dropdown.item_color_hover
            ));
            css_vars.push_str(&format!(
                "--thaw-font-color-disabled: {};",
                theme.dropdown.font_color_disabled
            ));
        });
        css_vars
    });

    let has_icon = use_context::<HasIcon>().expect("HasIcon not provided").0;

    if icon.get().is_some() {
        has_icon.set(true);
    }

    let on_click = move |event| {
        if disabled.get() {
            return;
        }
        let Some(callback) = on_click.as_ref() else {
            return;
        };
        callback.call(event);
    };

    view! {
        <div
            class=class_list![
                "thaw-dropdown-item", ("thaw-dropdown-item--disabled", move || disabled.get()),
                class.map(| c | move || c.get())
            ]

            style=move || css_vars.get()
            on:click=on_click
        >

            <OptionComp value=icon.get() let:icon>
                <Fallback slot>
                    <If cond=has_icon>
                        <Then slot>
                            <span style="width: 18px; margin-right: 8px"></span>
                        </Then>
                    </If>
                </Fallback>

                <Icon icon=icon style="font-size: 18px; margin-right: 8px"/>
            </OptionComp>
            <span style="flex-grow: 1">{label}</span>
        </div>
    }
}
