use leptos::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Caption1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || format!("thaw-caption-1 {:?}", class.get()));

    view! {
        <Text tag children class/>
    }
}

#[component]
pub fn Caption1Strong(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || format!("thaw-caption-1-strong {:?}", class.get()));

    view! {
        <Text tag children class/>
    }
}

#[component]
pub fn Body1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || format!("thaw-body-1 {:?}", class.get()));

    view! {
        <Text tag children class/>
    }
}

#[component]
pub fn Text(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    #[prop(optional)] code: bool,
    children: Children,
) -> impl IntoView {
    mount_style("text", include_str!("./text.css"));

    match tag {
        TextTag::B => todo!(),
        TextTag::Em => todo!(),
        TextTag::H1 => todo!(),
        TextTag::H2 => todo!(),
        TextTag::H3 => todo!(),
        TextTag::H4 => todo!(),
        TextTag::H5 => todo!(),
        TextTag::H6 => todo!(),
        TextTag::I => todo!(),
        TextTag::P => todo!(),
        TextTag::Pre => todo!(),
        TextTag::Span => view! {
            <span class=class_list!["thaw-text", class]>
                {children()}
            </span>
        },
        TextTag::Strong => todo!(),
    }
}

#[derive(Default)]
pub enum TextTag {
    B,
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
