use leptos::*;
use crate::use_nav_drawer;
use thaw_utils::{class_list, StoredMaybeSignal};

#[component]
pub fn NavItem(#[prop(into)] value: MaybeSignal<String>, children: Children) -> impl IntoView {
    let nav_drawer = use_nav_drawer();
    let value: StoredMaybeSignal<_> = value.into();
    let on_click = move |_| {
        let value = value.get();
        if nav_drawer.0.with(|key| key != &value) {
            nav_drawer.0.set(value);
        }
    };
    view! {
        <a class=class_list!["thaw-nav-item", ("thaw-nav-item--selected", move || nav_drawer.0.get() == value.get())]
            role="nav"
            type="navigation"
            on:click=on_click
        >
            {children()}
        </a>
    }
}
