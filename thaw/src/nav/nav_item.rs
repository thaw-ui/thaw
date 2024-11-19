use super::{NavCategoryInjection, NavDrawerInjection};
use crate::Icon;
use leptos::{either::Either, prelude::*};
use thaw_utils::{class_list, OptionModelWithValue};

#[component]
pub fn NavItem(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] icon: MaybeProp<icondata_core::Icon>,
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] href: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    let nav_drawer = NavDrawerInjection::expect_context();
    let nav_category = NavCategoryInjection::use_context();
    let on_click = move |_| {
        let value = value.get_untracked();
        if nav_drawer
            .selected_value
            .with_untracked(|selected_value| match selected_value {
                OptionModelWithValue::T(v) => v != &value,
                OptionModelWithValue::Option(v) => v.as_ref() != Some(&value),
            })
        {
            nav_drawer.selected_value.set(Some(value));
            nav_drawer
                .selected_category_value
                .set(nav_category.map(|c| c.value.get_untracked()));
        }
    };

    let children = || {
        view! {
            {move || {
                if let Some(icon) = icon.get() {
                    Either::Left(view! { <Icon icon=icon style="font-size: 20px" /> })
                } else {
                    Either::Right(())
                }
            }}
            {children()}
        }
    };

    let selected = Memo::new(move |_| {
        let selected = nav_drawer.selected_value.with(|selected_value| {
            value.with(|value| match selected_value {
                OptionModelWithValue::T(v) => v == value,
                OptionModelWithValue::Option(v) => v.as_ref() == Some(&value),
            })
        });

        if selected {
            if let Some(nav_category) = nav_category {
                let value = nav_category.value.get_untracked();
                if nav_drawer
                    .selected_category_value
                    .with_untracked(|selected_category_value| match selected_category_value {
                        OptionModelWithValue::T(v) => v != &value,
                        OptionModelWithValue::Option(v) => v.as_ref() != Some(&value),
                    })
                {
                    nav_drawer.selected_category_value.set(Some(value));
                }
            }
        }

        selected
    });

    if let Some(href) = href {
        Either::Left(view! {
            <a
                class=class_list![
                    "thaw-nav-item",
                    ("thaw-nav-item--selected", move || selected.get()),
                    class
                ]
                href=move || href.get()
                on:click=on_click
            >
                {children()}
            </a>
        })
    } else {
        Either::Right(view! {
            <button
                class=class_list![
                    "thaw-nav-item",
                    ("thaw-nav-item--selected", move || selected.get()),
                    class
                ]
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
    #[prop(into)] value: Signal<String>,
    #[prop(optional, into)] href: Option<Signal<String>>,
    children: Children,
) -> impl IntoView {
    let class = MaybeProp::derive(move || {
        format!("thaw-nav-sub-item {}", class.get().unwrap_or_default()).into()
    });

    if let Some(href) = href {
        Either::Left(view! { <NavItem class href icon value children /> })
    } else {
        Either::Right(view! { <NavItem class icon value children /> })
    }
}
