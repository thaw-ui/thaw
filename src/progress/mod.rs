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

#[component]
pub fn Progress(
    #[prop(into, optional)] percentage: MaybeSignal<f32>,
    #[prop(into, default = MaybeSignal::Static(true))] show_indicator: MaybeSignal<bool>,
    #[prop(into, optional)] indicator_placement: MaybeSignal<ProgressIndicatorPlacement>,
) -> impl IntoView {
    mount_style("progress", include_str!("./progress.css"));
    let theme = use_theme(Theme::light);
    let style = move || {
        let mut style = String::new();
        let percentage = percentage.get();
        let percentage = if percentage < 0.0 {
            0.0
        } else if percentage > 100.0 {
            100.0
        } else {
            percentage
        };
        style.push_str(&format!("width: {}%;", percentage));
        theme.with(|theme| {
            style.push_str(&format!(
                "--thaw-background-color: {};",
                theme.common.color_primary
            ));
        });
        style
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
        <div class="thaw-progress">
            <div class=class>
                <div class="thaw-progress__progress-inner" style=style>
                    <Show when=move || show_indicator.get() && indicator_placement.get() == ProgressIndicatorPlacement::Inside>
                        <div class="thaw-progress__indicator--inside">
                            {
                                move || {
                                    format!("{}%", percentage.get())
                                }
                            }
                        </div>
                    </Show>
                </div>
            </div>
            <Show when=move || show_indicator.get() && indicator_placement.get() == ProgressIndicatorPlacement::Outside>
                <div class="thaw-progress__indicator--outside">
                    {
                        move || {
                            format!("{}%", percentage.get())
                        }
                    }
                </div>
            </Show>
        </div>
    }
}
