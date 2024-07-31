use crate::{use_nav_drawer, Icon};
use leptos::{either::Either, prelude::*};
use thaw_utils::{class_list, StoredMaybeSignal};

#[component]
pub fn NavItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(optional, into)] href: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    let nav_drawer = use_nav_drawer();
    let value: StoredMaybeSignal<_> = value.into();
    let on_click = move |_| {
        let value = value.get();
        if nav_drawer.0.with(|key| key != &value) {
            nav_drawer.0.set(value);
        }
    };

    let children = || {
        view! {
            {
                move || {
                    if let Some(icon) = icon.get() {
                        Either::Left(view! {
                            <Icon icon=icon style="font-size: 20px"/>
                        })
                    } else {
                        Either::Right(())
                    }
                }
            }
            {children()}
        }
    };

    if let Some(href) = href {
        Either::Left(view! {
            <a
                class=class_list!["thaw-nav-item", ("thaw-nav-item--selected", move || nav_drawer.0.get() == value.get()), class]
                href=move || href.get()
                on:click=on_click
            >
                {children()}
            </a>
        })
    } else {
        Either::Right(view! {
            <button
                class=class_list!["thaw-nav-item", ("thaw-nav-item--selected", move || nav_drawer.0.get() == value.get()), class]
                on:click=on_click
            >
                {children()}
            </button>
        })
    }
}
