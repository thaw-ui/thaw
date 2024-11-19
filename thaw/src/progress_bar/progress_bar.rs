use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn ProgressBar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// A decimal number between 0 and 1 (or between 0 and max if given),
    /// which specifies how much of the task has been completed.
    #[prop(into, optional)]
    value: Signal<f64>,
    /// The maximum value, which indicates the task is complete.
    /// The ProgressBar bar will be full when value equals max.
    #[prop(default = 1.0.into(), optional)]
    max: Signal<f64>,
    /// ProgressBar color.
    #[prop(into, optional)]
    color: Signal<ProgressBarColor>,
) -> impl IntoView {
    mount_style("progress-bar", include_str!("./progress-bar.css"));

    let style = move || {
        let max = max.get();
        let value = value.get().max(0.0).min(max);
        format!("width: {:.02}%;", value / max * 100.0)
    };

    view! {
        <div
            class=class_list![
                "thaw-progress-bar",
                move || format!("thaw-progress-bar--{}", color.get().as_str()),
                class
            ]
            role="progressbar"
            aria_valuemax=move || max.get()
            aria-valuemin="0"
            aria-valuenow=move || value.get()
        >
            <div class="thaw-progress-bar__bar" style=style></div>
        </div>
    }
}

#[derive(Default, Clone)]
pub enum ProgressBarColor {
    #[default]
    Brand,
    Error,
    Warning,
    Success,
}

impl ProgressBarColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Success => "success",
        }
    }
}
