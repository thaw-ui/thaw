mod card_footer;
mod card_header;
mod card_preview;

pub use card_footer::*;
pub use card_header::*;
pub use card_preview::*;

use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/card/card.css");

#[component]
pub fn Card(#[prop(optional, into)] class: MaybeProp<String>, children: Children) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("card", include_str!("./card.css"));

    view! {
        <div class=class_list!["thaw-card", class] role="group">
            {children()}
        </div>
    }
}
