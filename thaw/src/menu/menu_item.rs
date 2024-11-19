use crate::{Icon, MenuInjection};
use leptos::prelude::*;
use thaw_components::{Fallback, If, OptionComp, Then};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn MenuItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The icon of the menu item.
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    /// The value of the menu item.
    #[prop(into)]
    value: Signal<String>,
    /// Whether the menu item is disabled.
    #[prop(optional, into)]
    disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));

    let MenuInjection {
        has_icon,
        on_select,
    } = MenuInjection::expect_context();

    if icon.with_untracked(|i| i.is_some()) {
        has_icon.set(true);
    }

    let on_click = move |_| {
        if disabled.get_untracked() {
            return;
        }

        on_select(value.get_untracked());
    };

    view! {
        <div
            class=class_list![
                "thaw-menu-item", ("thaw-menu-item--disabled", move || disabled.get()),
                class
            ]
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

                <Icon icon=icon style="font-size: 18px; margin-right: 8px" />
            </OptionComp>
            <span style="flex-grow: 1">{children()}</span>
        </div>
    }
}
