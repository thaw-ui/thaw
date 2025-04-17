use super::NavDrawerInjection;
use crate::Icon;
use leptos::{context::Provider, either::Either, prelude::*};
use thaw_components::CSSTransition;
use thaw_utils::class_list;

#[component]
pub fn NavCategory(
    #[prop(into)] value: Signal<String>,
    children: Children,
    nav_category_item: NavCategoryItem,
) -> impl IntoView {
    let nav_drawer = NavDrawerInjection::expect_context();
    let open = Memo::new(move |_| {
        value.with(|value| {
            nav_drawer
                .open_categories
                .with(|open_categories| open_categories.contains(value))
        })
    });
    let is_selected_category =
        Memo::new(move |_| value.with(|value| nav_drawer.is_selected_category(value)));

    let NavCategoryItem {
        class: item_class,
        icon: item_icon,
        children: item_children,
    } = nav_category_item;

    let on_nav_category_item_click = move |_| {
        nav_drawer.on_request_nav_category_item_toggle(value.get_untracked());
    };

    view! {
        <button
            class=class_list![
                "thaw-nav-category-item",
                ("thaw-nav-category-item--selected", move || is_selected_category.get()),
                item_class
            ]
            on:click=on_nav_category_item_click
            aria-expanded=move || if open.get() { "true" } else { "false" }
        >
            {move || {
                if let Some(icon) = item_icon.get() {
                    Either::Left(view! { <Icon icon=icon style="font-size: 20px" /> })
                } else {
                    Either::Right(())
                }
            }}
            {item_children()}
            <span aria-hidden="true" class="thaw-nav-category-item__expand-icon">
                <svg
                    fill="currentColor"
                    aria-hidden="true"
                    width="20"
                    height="20"
                    viewBox="0 0 20 20"
                    style=move || {
                        if open.get() {
                            "transform: rotate(90deg)"
                        } else {
                            "transform: rotate(0deg)"
                        }
                    }
                >
                    <path
                        d="M7.65 4.15c.2-.2.5-.2.7 0l5.49 5.46c.21.22.21.57 0 .78l-5.49 5.46a.5.5 0 0 1-.7-.7L12.8 10 7.65 4.85a.5.5 0 0 1 0-.7Z"
                        fill="currentColor"
                    ></path>
                </svg>
            </span>
        </button>
        <CSSTransition show=open name="thaw-nav-sub-item-group">
            <div class="thaw-nav-sub-item-group">
                <Provider value=NavCategoryInjection { value }>{children()}</Provider>
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

#[derive(Clone, Copy)]
pub(crate) struct NavCategoryInjection {
    pub value: Signal<String>,
}

impl NavCategoryInjection {
    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
