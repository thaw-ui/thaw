use super::ProgressColor;
use crate::{
    use_theme,
    utils::{class_list::class_list, mount_style, OptionalProp},
    Theme,
};
use leptos::*;

#[component]
pub fn ProgressCircle(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into, optional)] percentage: MaybeSignal<f32>,
    #[prop(into, optional)] color: MaybeSignal<ProgressColor>,
    #[prop(into, default = "120px".into())] size: MaybeSignal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("progress-circle", include_str!("./progress-circle.css"));
    let theme = use_theme(Theme::light);

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
    let rail_stroke_color =
        Memo::new(move |_| theme.with(|theme| theme.progress.background_color.clone()));

    let fill_path = rail_path.clone();
    let fill_stroke_dasharray = Memo::new(move |_| {
        let percentage = percentage.get();
        let percentage = if percentage < 0.0 {
            0.0
        } else if percentage > 100.0 {
            100.0
        } else {
            percentage
        };
        format!(
            "{}px {}px",
            f64::from(percentage / 100.0) * len,
            view_box_width * 8
        )
    });
    let fill_stroke_color =
        Memo::new(move |_| theme.with(|theme| color.get().theme_background_color(theme)));

    view! {
        <div
            class=class_list!["thaw-progress-circle", class.map(|c| move || c.get())]
            role="progressbar"
            aria-valuemax="100"
            aria-valuemin="0"
            aria-valuenow=move || percentage.get()
            style=("--thaw-fill-color", move || fill_stroke_color.get())
            style=("--thaw-size", move || size.get())

        >
            <svg viewBox="0 0 107 107">
                <g>
                    <path
                        d=rail_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke=move || rail_stroke_color.get()
                        style:stroke-dasharray=rail_stroke_dasharray
                    ></path>
                </g>
                <g>
                    <path
                        class=("thaw-progress-circle__fill", true)
                        class=("thaw-progress-circle__fill--empty", move || percentage.get() <= 0.0)
                        d=fill_path
                        stroke-width=stroke_width
                        stroke-linecap="round"
                        fill="none"
                        style:stroke="var(--thaw-fill-color)"
                        style:stroke-dasharray=move || fill_stroke_dasharray.get()
                    ></path>
                </g>
            </svg>
            {
                if let Some(children) = children {
                    view! {
                        <div class="thaw-progress-circle__content">
                            {children()}
                        </div>
                    }
                } else {
                    view! {
                        <div class="thaw-progress-circle__content thaw-progress-circle__content--text">
                            {move || percentage.get()}"%"
                        </div>
                    }
                }
            }
        </div>
    }
}
