use leptos::{either::EitherOf3, prelude::*};
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/code/code.css");

#[component]
pub fn Code(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] text: Option<String>,
    #[prop(optional, into)] inner_html: Option<String>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("code", include_str!("./code.css"));

    view! {
        <code class=class_list![
            "thaw-code",
            class
        ]>

            {if let Some(inner_html) = inner_html {
                EitherOf3::A(view! { <pre inner_html=inner_html></pre> })
            } else if let Some(text) = text {
                EitherOf3::B(view! { <pre>{text}</pre> })
            } else {
                EitherOf3::C(())
            }}

        </code>
    }
}
