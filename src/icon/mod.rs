// copy https://github.com/Carlosted/leptos-icons
// leptos updated version causes leptos_icons error
pub use icondata::*;
use leptos::SignalGet;

/// The Icon component.
#[leptos::component]
pub fn Icon(
    cx: leptos::Scope,
    /// The icon to show.
    #[prop(into)]
    icon: leptos::MaybeSignal<Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: Option<leptos::MaybeSignal<String>>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: Option<leptos::MaybeSignal<String>>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: Option<leptos::MaybeSignal<String>>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<leptos::MaybeSignal<String>>,
) -> impl leptos::IntoView {
    let icon: IconData = icon.get().into();

    let mut svg = leptos::svg::svg(cx);
    if let Some(classes) = class {
        svg = svg.classes(classes.get());
    }
    // The style set by the user overrides the style set by the icon.
    svg = match (style, icon.style) {
        (Some(a), Some(b)) => svg.attr("style", format!("{b} {}", a.get())),
        (Some(a), None) => svg.attr("style", a.get()),
        (None, Some(b)) => svg.attr("style", b),
        (None, None) => svg,
    };
    if let Some(x) = icon.x {
        svg = svg.attr("x", x);
    }
    if let Some(y) = icon.y {
        svg = svg.attr("x", y);
    }
    // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
    svg = svg.attr(
        "width",
        leptos::Attribute::String(match (width, icon.width) {
            (Some(a), Some(_b)) => std::borrow::Cow::from(a.get()),
            (Some(a), None) => std::borrow::Cow::from(a.get()),
            (None, Some(_b)) => std::borrow::Cow::from("1em"),
            (None, None) => std::borrow::Cow::from("1em"),
        }),
    );
    svg = svg.attr(
        "height",
        leptos::Attribute::String(match (height, icon.height) {
            (Some(a), Some(_b)) => std::borrow::Cow::from(a.get()),
            (Some(a), None) => std::borrow::Cow::from(a.get()),
            (None, Some(_b)) => std::borrow::Cow::from("1em"),
            (None, None) => std::borrow::Cow::from("1em"),
        }),
    );
    if let Some(view_box) = icon.view_box {
        svg = svg.attr("viewBox", view_box);
    }
    if let Some(stroke_linecap) = icon.stroke_linecap {
        svg = svg.attr("stroke-linecap", stroke_linecap);
    }
    if let Some(stroke_linejoin) = icon.stroke_linejoin {
        svg = svg.attr("stroke-linejoin", stroke_linejoin);
    }
    if let Some(stroke_width) = icon.stroke_width {
        svg = svg.attr("stroke-width", stroke_width);
    }
    if let Some(stroke) = icon.stroke {
        svg = svg.attr("stroke", stroke);
    }
    svg = svg.attr("fill", icon.fill.unwrap_or("currentColor"));
    svg = svg.attr("role", "graphics-symbol");
    svg = svg.inner_html(icon.data);
    leptos::IntoView::into_view(svg, cx)
}
