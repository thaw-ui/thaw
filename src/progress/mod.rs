use crate::utils::{mount_style::mount_style, StoredMaybeSignal};
use leptos::*;

#[component]
pub fn Progress(
    #[prop(optional, into)] left_tip: MaybeSignal<String>,
    #[prop(optional, into)] right_tip: MaybeSignal<String>,
    percentage: ReadSignal<f64>,
) -> impl IntoView {
    mount_style("progress", include_str!("./progress.css"));
    let style = move || format!("width: {}%", percentage.get());
    let left_tip: StoredMaybeSignal<_> = left_tip.into();
    let right_tip: StoredMaybeSignal<_> = right_tip.into();

    view! {
        <div class="melt-progress">
            <span class="melt-progress__tip-left">
                <Show when=move || left_tip.with(|v| !v.is_empty())>{move || left_tip.get()}</Show>
            </span>
            <span class="melt-progress__progress">
                <span class="melt-progress__progress-inner" style=style>
                    <span class="melt-progress__progress-circle"></span>
                </span>
            </span>
            <span class="melt-progress__tip-right">
                <Show when=move || {
                    right_tip.with(|v| !v.is_empty())
                }>{move || right_tip.get()}</Show>
            </span>
        </div>
    }
}
