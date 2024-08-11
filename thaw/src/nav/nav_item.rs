use super::NavDrawerInjection;
use crate::Icon;
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
    let nav_drawer = NavDrawerInjection::expect_context();
    let value: StoredMaybeSignal<_> = value.into();
    let on_click = move |_| {
        let value = value.get_untracked();
        if nav_drawer
            .selected_value
            .with_untracked(|selected_value| selected_value != Some(&value))
        {
            nav_drawer.selected_value.set(Some(value));
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

    let selected = Memo::new(move |_| {
        nav_drawer
            .selected_value
            .with(|selected_value| value.with(|value| selected_value == Some(value)))
    });

    if let Some(href) = href {
        Either::Left(view! {
            <a
                class=class_list!["thaw-nav-item", ("thaw-nav-item--selected", move || selected.get()), class]
                href=move || href.get()
                on:click=on_click
            >
                {children()}
            </a>
        })
    } else {
        Either::Right(view! {
            <button
                class=class_list!["thaw-nav-item", ("thaw-nav-item--selected", move || selected.get()), class]
                on:click=on_click
            >
                {children()}
            </button>
        })
    }
}

#[component]
pub fn NavSubItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(into)] value: MaybeSignal<String>,
    #[prop(optional, into)] href: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    let class = MaybeProp::derive(move || {
        format!("thaw-nav-sub-item {}", class.get().unwrap_or_default()).into()
    });

    if let Some(href) = href {
        Either::Left(view! { <NavItem class href icon value children/> })
    } else {
        Either::Right(view! { <NavItem class icon value children/> })
    }
}
