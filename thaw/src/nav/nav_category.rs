use super::NavDrawerInjection;
use crate::Icon;
use leptos::{either::Either, html, prelude::*};
use thaw_components::CSSTransition;
use thaw_utils::{class_list, StoredMaybeSignal};

#[component]
pub fn NavCategory(
    #[prop(into)] value: MaybeSignal<String>,
    children: Children,
    nav_category_item: NavCategoryItem,
) -> impl IntoView {
    let nav_drawer = NavDrawerInjection::expect_context();
    let value: StoredMaybeSignal<_> = value.into();
    let group_ref = NodeRef::<html::Div>::new();
    let on_click = move |_| {
        let value = value.get_untracked();
        if nav_drawer
            .selected_category_value
            .with_untracked(|selected_category_value| selected_category_value != Some(&value))
        {
            nav_drawer.selected_category_value.set(Some(value));
        }
    };

    let is_show_group = Memo::new(move |_| {
        nav_drawer
            .selected_category_value
            .with_untracked(|selected_category_value| {
                value.with_untracked(|value| selected_category_value == Some(value))
            })
    });

    let NavCategoryItem {
        class: item_class,
        icon: item_icon,
        children: item_children,
    } = nav_category_item;

    view! {
        <button
            class=class_list!["thaw-nav-category-item", item_class]
            on:click=on_click
            aria-expanded=move || is_show_group.get()
        >
            {
                move || {
                    if let Some(icon) = item_icon.get() {
                        Either::Left(view! {
                            <Icon icon=icon style="font-size: 20px"/>
                        })
                    } else {
                        Either::Right(())
                    }
                }
            }
            {item_children()}
            <span aria-hidden="true" class="thaw-nav-category-item__expand-icon">
                <svg
                    fill="currentColor"
                    aria-hidden="true"
                    width="20"
                    height="20"
                    viewBox="0 0 20 20"
                    style=move || if is_show_group.get() {
                        "transform: rotate(90deg)"
                    } else {
                        "transform: rotate(0deg)"
                    }
                >
                    <path d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z" fill="currentColor"></path>
                </svg>
            </span>
        </button>
        <CSSTransition
            node_ref=group_ref
            show=is_show_group
            name="thaw-nav-sub-item-group"
            let:display
        >
            <div
                class="thaw-nav-sub-item-group"
                node_ref=group_ref
                style=move || display.get().unwrap_or_default()
            >
                {children()}
            </div>
        </CSSTransition>
    }
}

#[slot]
pub struct NavCategoryItem {
    #[prop(optional, into)]
    class: MaybeProp<String>,
    #[prop(optional, into)]
    icon: MaybeProp<icondata_core::Icon>,
    children: Children,
}
