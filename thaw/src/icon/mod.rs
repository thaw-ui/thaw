// copy https://github.com/Carlosted/leptos-icons
// leptos updated version causes leptos_icons error
use leptos::*;

/// The Icon component.
#[component]
pub fn Icon(
    /// The icon to render.
    #[prop(into)]
    icon: MaybeSignal<icondata_core::Icon>,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: Option<MaybeSignal<String>>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: Option<MaybeSignal<String>>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: Option<MaybeSignal<String>>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<MaybeSignal<String>>,
) -> impl IntoView {
    let svg = move || {
        let icon = icon.get();
        let mut svg = svg::svg();
        if let Some(class) = class.clone() {
            svg = svg.attr("class", move || class.get())
        }
        let mut svg = match (style.clone(), icon.style) {
            (Some(a), Some(b)) => svg.attr("style", move || format!("{b} {}", a.get())),
            (Some(a), None) => svg.attr("style", move || a.get()),
            (None, Some(b)) => svg.attr("style", b),
            (None, None) => svg,
        };

        if let Some(x) = icon.x {
            svg = svg.attr("x", x);
        }
        if let Some(y) = icon.y {
            svg = svg.attr("x", y);
        }
        //// The style set by the user overrides the style set by the icon.
        // We ignore the width and height attributes of the icon, even if the user hasn't specified any.
        svg = svg.attr(
            "width",
            match (width.clone(), icon.width) {
                (Some(a), _) => (move || a.get()).into_attribute(),
                _ => "1em".into_attribute(),
            },
        );
        svg = svg.attr(
            "height",
            match (height.clone(), icon.height) {
                (Some(a), _) => (move || a.get()).into_attribute(),
                _ => "1em".into_attribute(),
            },
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
        svg
    };
    svg.into_view()
}
