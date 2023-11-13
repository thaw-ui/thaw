use crate::utils::{mount_style, StoredMaybeSignal};
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
        <div class="thaw-progress">
            <span class="thaw-progress__tip-left">
                <Show when=move || left_tip.with(|v| !v.is_empty())>{move || left_tip.get()}</Show>
            </span>
            <span class="thaw-progress__progress">
                <span class="thaw-progress__progress-inner" style=style>
                    <span class="thaw-progress__progress-circle"></span>
                </span>
            </span>
            <span class="thaw-progress__tip-right">
                <Show when=move || {
                    right_tip.with(|v| !v.is_empty())
                }>{move || right_tip.get()}</Show>
            </span>
        </div>
    }
}
