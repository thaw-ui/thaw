use crate::Scrollbar;
use leptos::{either::Either, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn DrawerBody(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Whether to use native scrollbar on itself.
    #[prop(optional)]
    native_scrollbar: bool,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list![
            "thaw-drawer-body", class
        ]>
            {if native_scrollbar {
                Either::Left(children())
            } else {
                Either::Right(view! { <Scrollbar>{children()}</Scrollbar> })
            }}
        </div>
    }
}
