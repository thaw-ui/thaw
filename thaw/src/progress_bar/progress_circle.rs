use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn ProgressCircle(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Percentage value.
    #[prop(into, optional)]
    value: MaybeSignal<f64>,
    /// ProgressCircle color.
    #[prop(into, optional)]
    color: MaybeSignal<ProgressCircleColor>,
    /// ProgressCircle size.
    #[prop(into, default = "120px".into())]
    size: MaybeSignal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("progress-circle", include_str!("./progress-circle.css"));

    let stroke_width = 7;
    let view_box_width = 100;

    let radius = 50;
    let begin_position_x = 0;
    let begin_position_y = radius;
    let end_position_x = 0;
    let end_position_y = 2 * radius;
    let center_x = 50 + stroke_width / 2;
    let rail_path = format!("M {center_x},{center_x} m {begin_position_x},{begin_position_y} a {radius},{radius} 0 1 1 {end_position_x},{} a {radius},{radius} 0 1 1 {},{end_position_y}", -end_position_y, -end_position_x);

    let len = std::f64::consts::PI * 2.0 * f64::from(radius);
    let rail_stroke_dasharray = format!("{len}px {}px", view_box_width * 8);

    let fill_path = rail_path.clone();
    let fill_stroke_dasharray = Memo::new(move |_| {
        let percentage = value.get().max(0.0).min(100.0);

        format!("{}px {}px", percentage / 100.0 * len, view_box_width * 8)
    });
    let fill_stroke_color = move || match color.get() {
        ProgressCircleColor::Brand => "var(--colorCompoundBrandBackground)",
        ProgressCircleColor::Error => "var(--colorPaletteRedBackground3)",
        ProgressCircleColor::Warning => "var(--colorPaletteDarkOrangeBackground3)",
        ProgressCircleColor::Success => "var(--colorPaletteGreenBackground3)",
    };

    view! {
        <div
            class=class_list!["thaw-progress-circle", class]
            role="progressbar"
            aria-valuemax="100"
            aria-valuemin="0"
            aria-valuenow=move || value.get()
            style=("--thaw-size", move || size.get())
        >

            <svg viewBox="0 0 107 107">
                <g>
                    <path
                        d=rail_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke="var(--colorNeutralBackground6)"
                        style:stroke-dasharray=rail_stroke_dasharray
                    ></path>
                </g>
                <g>
                    <path
                        class=("thaw-progress-circle__fill", true)
                        class=("thaw-progress-circle__fill--empty", move || value.get() <= 0.0)
                        d=fill_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke=fill_stroke_color
                        style:stroke-dasharray=move || fill_stroke_dasharray.get()
                    ></path>
                </g>
            </svg>

            {if let Some(children) = children {
                view! { <div class="thaw-progress-circle__content">{children()}</div> }.into_any()
            } else {
                view! {
                    <div class="thaw-progress-circle__content thaw-progress-circle__content--text">
                        {move || value.get()} "%"
                    </div>
                }.into_any()
            }}

        </div>
    }
}

#[derive(Default, Clone)]
pub enum ProgressCircleColor {
    #[default]
    Brand,
    Error,
    Warning,
    Success,
}
