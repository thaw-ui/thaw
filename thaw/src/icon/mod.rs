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
    /// Callback when clicking on the icon.
    #[prop(optional, into)]
    on_click: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    let icon_style = RwSignal::new(None);
    let icon_x = RwSignal::new(None);
    let icon_y = RwSignal::new(None);
    let icon_width = RwSignal::new(None);
    let icon_height = RwSignal::new(None);
    let icon_view_box = RwSignal::new(None);
    let icon_stroke_linecap = RwSignal::new(None);
    let icon_stroke_linejoin = RwSignal::new(None);
    let icon_stroke_width = RwSignal::new(None);
    let icon_stroke = RwSignal::new(None);
    let icon_fill = RwSignal::new(None);
    let icon_data = RwSignal::new(None);
    let on_click = move |ev| {
        if let Some(click) = on_click.as_ref() {
            click.call(ev);
        }
    };

    create_isomorphic_effect(move |_| {
        let icon = icon.get();

        let style = match (style.clone(), icon.style) {
            (Some(a), Some(b)) => Some(Memo::new(move |_| format!("{b} {}", a.get())).into()),
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b.into()),
            (None, None) => None,
        };
        icon_style.set(style);

        icon_x.set(icon.x.map(|x| x.into_attribute()));
        icon_y.set(icon.y.map(|y| y.into_attribute()));

        let width = match (width.clone(), icon.width) {
            (Some(a), _) => a,
            _ => "1em".into(),
        };
        icon_width.set(Some(width));

        let height = match (height.clone(), icon.height) {
            (Some(a), _) => a,
            _ => "1em".into(),
        };
        icon_height.set(Some(height));

        icon_view_box.set(icon.view_box.map(|view_box| view_box.into_attribute()));
        icon_stroke_linecap.set(icon.stroke_linecap.map(|a| a.into_attribute()));
        icon_stroke_linejoin.set(icon.stroke_linejoin.map(|a| a.into_attribute()));
        icon_stroke_width.set(icon.stroke_width.map(|a| a.into_attribute()));
        icon_stroke.set(icon.stroke.map(|a| a.into_attribute()));
        icon_fill.set(Some(icon.fill.unwrap_or("currentColor").into_attribute()));
        icon_data.set(Some(icon.data.into_attribute()));
    });

    view! {
        <svg
            class=class.map(|c| c.get())
            style=move || take_signal(icon_style)
            x=move || take(icon_x)
            y=move || take(icon_y)
            width=move || take_signal(icon_width)
            height=move || take_signal(icon_height)
            viewBox=move || take(icon_view_box)
            stroke-linecap=move || take(icon_stroke_linecap)
            stroke-linejoin=move || take(icon_stroke_linejoin)
            stroke-width=move || take(icon_stroke_width)
            stroke=move || take(icon_stroke)
            fill=move || take(icon_fill)
            inner_html=move || take(icon_data)
            on:click=on_click
        ></svg>
    }
}

fn take_signal(signal: RwSignal<Option<MaybeSignal<String>>>) -> Option<String> {
    signal.with(|s| match s {
        Some(MaybeSignal::Static(value)) => Some(value.clone()),
        Some(MaybeSignal::Dynamic(signal)) => Some(signal.get()),
        _ => None,
    })
}

fn take(signal: RwSignal<Option<Attribute>>) -> Option<Attribute> {
    signal.track();
    signal.try_update_untracked(|value| value.take()).flatten()
}
