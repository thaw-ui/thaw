use crate::{components::*, utils::mount_style::mount_style};
use leptos::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] left_tip: MaybeSignal<&'static str>,
    #[prop(optional, into)] right_tip: MaybeSignal<&'static str>,
    percentage: ReadSignal<f64>,
) -> impl IntoView {
    mount_style("progress", include_str!("./progress.css"));
    let style = move || format!("width: {}%", percentage.get());
    view! {
        <div class="melt-progress">
            <span class="melt-progress__tip-left">
                <If cond=MaybeSignal::derive(move || !left_tip.get().is_empty())>
                    <Then slot>{left_tip.get()}</Then>
                </If>
            </span>
            <span class="melt-progress__progress">
                <span class="melt-progress__progress-inner" style=style>
                    <span class="melt-progress__progress-circle"></span>
                </span>
            </span>
            <span class="melt-progress__tip-right">
                <If cond=MaybeSignal::derive(move || !right_tip.get().is_empty())>
                    <Then slot>{right_tip.get()}</Then>
                </If>
            </span>
        </div>
    }
}
