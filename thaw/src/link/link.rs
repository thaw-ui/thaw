use leptos::{either::EitherOf3, prelude::*};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Link(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] span: bool,
    /// If true, changes styling when the link is being used alongside other text content.
    #[prop(optional, into)]
    inline: MaybeSignal<bool>,
    #[prop(optional, into)] href: Option<MaybeSignal<String>>,
    /// Whether the link is disabled.
    #[prop(optional, into)]
    disabled: MaybeSignal<bool>,
    /// When set, allows the link to be focusable even when it has been disabled.
    #[prop(optional, into)]
    disabled_focusable: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("link", include_str!("./link.css"));

    let link_disabled = Memo::new(move |_| disabled.get() || disabled_focusable.get());
    let class = class_list![
        "thaw-link",
        ("thaw-link--inline", move || inline.get()),
        ("thaw-link--disabled", move || link_disabled.get()),
        ("thaw-link--disabled-focusable", move || link_disabled.get()),
        class
    ];

    let tabindex = Memo::new(move |_| {
        if disabled_focusable.get() {
            Some("0")
        } else if disabled.get() {
            Some("-1")
        } else {
            None
        }
    });

    if let Some(href) = href {
        EitherOf3::A(view! {
            <a
                role="link"
                class=class
                href=href
                tabindex=tabindex
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </a>
        })
    } else if span {
        EitherOf3::B(view! {
            <span class=class>
                {children()}
            </span>
        })
    } else {
        EitherOf3::C(view! {
            <button
                class=class
                disabled=move || disabled.get().then_some("")
                aria-disabled=move || link_disabled.get().then_some("true")
            >
                {children()}
            </button>
        })
    }
}
