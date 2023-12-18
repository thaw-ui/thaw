use leptos::*;

#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::ssr_class;

#[component]
pub fn Image(
    #[prop(optional, into)] src: MaybeSignal<String>,
    #[prop(optional, into)] alt: MaybeSignal<String>,
    #[prop(optional, into)] width: MaybeSignal<String>,
    #[prop(optional, into)] height: MaybeSignal<String>,
    #[prop(optional, into)] border_radius: MaybeSignal<String>,
    #[prop(optional, into)] object_fit: MaybeSignal<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    let style = move || {
        let mut style = String::new();

        let width = width.get();
        if !width.is_empty() {
            style.push_str(&format!("width: {width};"))
        }

        let height = height.get();
        if !height.is_empty() {
            style.push_str(&format!("height: {height};"))
        }

        let border_radius = border_radius.get();
        if !border_radius.is_empty() {
            style.push_str(&format!("border-radius: {border_radius};"))
        }

        style
    };

    let ssr_class = ssr_class(&class);
    view! {
        <img class=ssr_class use:dyn_classes=class
            src=move || src.get()
            alt=move || alt.get()
            style=style
            object_fit=move || object_fit.get()
        />
    }
}


