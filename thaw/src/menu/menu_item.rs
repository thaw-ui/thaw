use crate::{
    menu::{HasIcon, OnSelect},
    Icon,
};
use leptos::prelude::*;
use thaw_components::{Fallback, If, OptionComp, Then};
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal};

#[component]
pub fn MenuItem(
    #[prop(optional, into)] icon: OptionalMaybeSignal<icondata_core::Icon>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));

    let has_icon = use_context::<HasIcon>().expect("HasIcon not provided").0;

    if icon.get().is_some() {
        has_icon.set(true);
    }

    let on_select = use_context::<OnSelect>().expect("OnSelect not provided").0;

    let on_click = move |_| {
        if disabled.get() {
            return;
        }
        on_select(value.get());
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

                <Icon icon=icon style="font-size: 18px; margin-right: 8px"/>
            </OptionComp>
            <span style="flex-grow: 1">{children()}</span>
        </div>
    }
}
