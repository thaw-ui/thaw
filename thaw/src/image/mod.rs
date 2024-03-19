use leptos::*;
use thaw_utils::OptionalProp;

#[component]
pub fn Image(
    #[prop(optional, into)] src: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] alt: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] width: MaybeSignal<String>,
    #[prop(optional, into)] height: MaybeSignal<String>,
    #[prop(optional, into)] border_radius: MaybeSignal<String>,
    #[prop(optional, into)] object_fit: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
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

    view! {
        <img
            class=class.map(|c| move || c.get())
            src=src.map(|s| move || s.get())
            alt=alt.map(|a| move || a.get())
            style=style
            object_fit=object_fit.map(|o| move || o.get())
        />
    }
}
