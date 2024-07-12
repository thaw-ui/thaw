use leptos::{either::Either, ev, prelude::*};
use send_wrapper::SendWrapper;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Tag(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] closable: MaybeSignal<bool>,
    #[prop(optional, into)] on_close: Option<Callback<SendWrapper<ev::MouseEvent>>>,
    children: Children,
) -> impl IntoView {
    mount_style("tag", include_str!("./tag.css"));

    let on_close = move |event| {
        let Some(callback) = on_close.as_ref() else {
            return;
        };
        callback.call(SendWrapper::new(event));
    };

    view! {
        <span
            class=class_list!["thaw-tag", ("thaw-tag--closable", move || closable.get()), class.map(| c | move || c.get())]

        >
            <span class="thaw-tag__primary-text">{children()}</span>

            {move || {
                if closable.get() {
                    Either::Left(view! {
                        <button class="thaw-tag__close" on:click=on_close>
                            <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                <path d="m4.09 4.22.06-.07a.5.5 0 0 1 .63-.06l.07.06L10 9.29l5.15-5.14a.5.5 0 0 1 .63-.06l.07.06c.18.17.2.44.06.63l-.06.07L10.71 10l5.14 5.15c.18.17.2.44.06.63l-.06.07a.5.5 0 0 1-.63.06l-.07-.06L10 10.71l-5.15 5.14a.5.5 0 0 1-.63.06l-.07-.06a.5.5 0 0 1-.06-.63l.06-.07L9.29 10 4.15 4.85a.5.5 0 0 1-.06-.63l.06-.07-.06.07Z" fill="currentColor"></path>
                            </svg>
                        </button>
                    })
                } else {
                    Either::Right(())
                }
            }}

        </span>
    }
}
