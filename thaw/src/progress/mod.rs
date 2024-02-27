mod progress_circle;
mod theme;

pub use progress_circle::ProgressCircle;
pub use theme::ProgressTheme;

use crate::{use_theme, utils::mount_style, Theme};
use leptos::*;

#[derive(Default, Clone, PartialEq)]
pub enum ProgressIndicatorPlacement {
    #[default]
    Outside,
    Inside,
}

impl Copy for ProgressIndicatorPlacement {}

impl ProgressIndicatorPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProgressIndicatorPlacement::Outside => "outside",
            ProgressIndicatorPlacement::Inside => "inside",
        }
    }
}

#[derive(Default, Clone)]
pub enum ProgressColor {
    #[default]
    Primary,
    Success,
    Warning,
    Error,
}

impl ProgressColor {
    fn theme_background_color(&self, theme: &Theme) -> String {
        match self {
            Self::Primary => theme.common.color_primary.clone(),
            Self::Success => theme.common.color_success.clone(),
            Self::Warning => theme.common.color_warning.clone(),
            Self::Error => theme.common.color_error.clone(),
        }
    }
}

#[component]
pub fn Progress(
    #[prop(into, optional)] percentage: MaybeSignal<f32>,
    #[prop(into, optional)] color: MaybeSignal<ProgressColor>,
    #[prop(into, default = MaybeSignal::Static(true))] show_indicator: MaybeSignal<bool>,
    #[prop(into, optional)] indicator_placement: MaybeSignal<ProgressIndicatorPlacement>,
) -> impl IntoView {
    mount_style("progress", include_str!("./progress.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.progress.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-inner-background-color: {};",
                color.get().theme_background_color(theme)
            ));
        });
        css_vars
    });
    let style = move || {
        let percentage = percentage.get();
        let percentage = if percentage < 0.0 {
            0.0
        } else if percentage > 100.0 {
            100.0
        } else {
            percentage
        };
        format!("width: {}%;", percentage)
    };

    let class = move || {
        let mut class = String::from("thaw-progress__progress");

        class.push_str(&format!(
            " thaw-progress__progress--indicator-{}",
            indicator_placement.get().as_str()
        ));

        class
    };

    view! {
        <div class="thaw-progress" style=move || css_vars.get()>
            <div class=class>
                <div class="thaw-progress__progress-inner" style=style>
                    <Show when=move || {
                        show_indicator.get()
                            && indicator_placement.get() == ProgressIndicatorPlacement::Inside
                    }>
                        <div class="thaw-progress__indicator--inside">

                            {move || { format!("{}%", percentage.get()) }}

                        </div>
                    </Show>
                </div>
            </div>
            <Show when=move || {
                show_indicator.get()
                    && indicator_placement.get() == ProgressIndicatorPlacement::Outside
            }>
                <div class="thaw-progress__indicator--outside">

                    {move || { format!("{}%", percentage.get()) }}

                </div>
            </Show>
        </div>
    }
}
