// copy https://github.com/Carlosted/leptos-icons
// leptos updated version causes leptos_icons error
mod icons;

pub(crate) use icons::*;

use leptos::{ev, prelude::*};
use thaw_utils::{class_list, mount_style, ArcOneCallback};

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: MaybeSignal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    width: MaybeProp<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into, default = "1em".into())]
    height: MaybeProp<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: MaybeProp<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<MaybeSignal<String>>,
    /// Callback when clicking on the icon.
    #[prop(optional, into)]
    on_click: Option<ArcOneCallback<ev::MouseEvent>>,
) -> impl IntoView {
    mount_style("icon", include_str!("./icon.css"));
    let class = class_list!["thaw-icon", class];

    move || {
        let icon = icon.get();

        let style = match (style.clone(), icon.style) {
            (Some(a), Some(b)) => Some(ArcMemo::new(move |_| format!("{b} {}", a.get())).into()),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b.into()),
            (None, None) => None,
        };
        let width = width.clone();
        let height = height.clone();
        let on_click = on_click.clone();
        let on_click = move |ev| {
            if let Some(click) = on_click.as_ref() {
                click(ev);
            }
        };

        view! {
            <svg
                class=class.clone()
                style=move || if let Some(s) = style.as_ref() { s.get() } else { String::new() }
                x=icon.x
                y=icon.y
                width=move || width.get()
                height=move || height.get()
                viewBox=icon.view_box
                stroke-linecap=icon.stroke_linecap
                stroke-linejoin=icon.stroke_linejoin
                stroke-width=icon.stroke_width
                stroke=icon.stroke
                fill=icon.fill.unwrap_or("currentColor")
                inner_html=icon.data
                on:click=on_click
            ></svg>
        }
    }
}
