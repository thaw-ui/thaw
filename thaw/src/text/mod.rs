use leptos::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Caption1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class =
        Signal::derive(move || format!("thaw-caption-1 {}", class.get().unwrap_or_default()));

    view! {
        <Text tag children class style/>
    }
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

    view! {
        <Text tag children class style/>
    }
}

#[component]
pub fn Body1(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    children: Children,
) -> impl IntoView {
    let class = Signal::derive(move || format!("thaw-body-1 {}", class.get().unwrap_or_default()));

    view! {
        <Text tag children class style/>
    }
}

#[component]
pub fn Text(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    #[prop(optional)] tag: TextTag,
    #[prop(optional)] code: bool,
    children: Children,
) -> impl IntoView {
    mount_style("text", include_str!("./text.css"));

    match tag {
        TextTag::B => view! {
            <b class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </b>
        }
        .into_view(),
        TextTag::Em => view! {
            <em class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </em>
        }
        .into_view(),
        TextTag::H1 => view! {
            <h1 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h1>
        }
        .into_view(),
        TextTag::H2 => view! {
            <h2 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h2>
        }
        .into_view(),
        TextTag::H3 => view! {
            <h3 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h3>
        }
        .into_view(),
        TextTag::H4 => view! {
            <h4 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h4>
        }
        .into_view(),
        TextTag::H5 => view! {
            <h5 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h5>
        }
        .into_view(),
        TextTag::H6 => view! {
            <h6 class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </h6>
        }
        .into_view(),
        TextTag::I => view! {
            <i class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </i>
        }
        .into_view(),
        TextTag::P => view! {
            <p class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </p>
        }
        .into_view(),
        TextTag::Pre => view! {
            <pre class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </pre>
        }
        .into_view(),
        TextTag::Span => view! {
            <span class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </span>
        }
        .into_view(),
        TextTag::Strong => view! {
            <strong class=class_list!["thaw-text", class] style=move || style.get()>
                {children()}
            </strong>
        }
        .into_view(),
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
