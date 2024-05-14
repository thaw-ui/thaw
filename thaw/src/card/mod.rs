mod card_footer;
mod card_header;
mod card_preview;

pub use card_footer::*;
pub use card_header::*;
pub use card_preview::*;

use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[slot]
pub struct CardHeaderExtra {
    children: Children,
}

#[slot]
pub struct CardFooter {
    #[prop(default = leptos::MaybeSignal::Static(true), into)]
    if_: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[component]
pub fn Card(
    #[prop(optional, into)] title: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] card_header_extra: Option<CardHeaderExtra>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    children: Children,
    #[prop(optional)] card_footer: Option<CardFooter>,
) -> impl IntoView {
    mount_style("card", include_str!("./card.css"));

    view! {
        <div
            class=class_list!["thaw-card", class.map(| c | move || c.get())]
            role="group"
        >
            {children()}
        </div>
    }
}
