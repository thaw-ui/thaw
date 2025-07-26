use leptos::{prelude::*, tachys::view::any_view::IntoAny};
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/text/text.css");

#[component]
pub fn Caption1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class =
        Signal::derive(move || format!("thaw-caption-1 {}", class.get().unwrap_or_default()));

    view! { <Text tag children class style /> }
}

#[component]
pub fn Caption1Strong(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || {
        format!("thaw-caption-1-strong {}", class.get().unwrap_or_default())
    });

    view! { <Text tag children class style /> }
}

#[component]
pub fn Body1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || format!("thaw-body-1 {}", class.get().unwrap_or_default()));

    view! { <Text tag children class style /> }
}

#[component]
pub fn Text(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("text", include_str!("./text.css"));

    let class = class_list!["thaw-text", class];
    let style = move || style.get();

    match tag {
        TextTag::B => view! {
            <b class=class style=style>
                {children()}
            </b>
        }
        .into_any(),
        TextTag::Code => view! {
            <code class=class style=style>
                {children()}
            </code>
        }
        .into_any(),
        TextTag::Em => view! {
            <em class=class style=style>
                {children()}
            </em>
        }
        .into_any(),
        TextTag::H1 => view! {
            <h1 class=class style=style>
                {children()}
            </h1>
        }
        .into_any(),
        TextTag::H2 => view! {
            <h2 class=class style=style>
                {children()}
            </h2>
        }
        .into_any(),
        TextTag::H3 => view! {
            <h3 class=class style=style>
                {children()}
            </h3>
        }
        .into_any(),
        TextTag::H4 => view! {
            <h4 class=class style=style>
                {children()}
            </h4>
        }
        .into_any(),
        TextTag::H5 => view! {
            <h5 class=class style=style>
                {children()}
            </h5>
        }
        .into_any(),
        TextTag::H6 => view! {
            <h6 class=class style=style>
                {children()}
            </h6>
        }
        .into_any(),
        TextTag::I => view! {
            <i class=class style=style>
                {children()}
            </i>
        }
        .into_any(),
        TextTag::P => view! {
            <p class=class style=style>
                {children()}
            </p>
        }
        .into_any(),
        TextTag::Pre => view! {
            <pre class=class style=style>
                {children()}
            </pre>
        }
        .into_any(),
        TextTag::Span => view! {
            <span class=class style=style>
                {children()}
            </span>
        }
        .into_any(),
        TextTag::Strong => view! {
            <strong class=class style=style>
                {children()}
            </strong>
        }
        .into_any(),
    }
}

#[derive(Default, PartialEq)]
pub enum TextTag {
    B,
    Code,
    Em,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    I,
    P,
    Pre,
    #[default]
    Span,
    Strong,
}
