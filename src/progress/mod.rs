use crate::{utils::mount_style::mount_style, components::*};
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Progress(
    cx: Scope,
    #[prop(optional, into)] left_tip: MaybeSignal<&'static str>,
    #[prop(optional, into)] right_tip: MaybeSignal<&'static str>,
    percentage: ReadSignal<f64>,
) -> impl IntoView {
    let class_name = mount_style("progress", || style_sheet_str!("./src/progress/progress.css"));
    let style = move || format!("width: {}%", percentage.get());
    view! {
        cx, class=class_name,
        <div class="melt-progress">
            <span class="melt-progress__tip-left">
                <If cond=MaybeSignal::derive(cx, move || !left_tip.get().is_empty())>
                    <Then slot>
                        { left_tip.get() }
                    </Then>
                </If>
            </span>
            <span class="melt-progress__progress">
                <span class="melt-progress__progress-inner" style=style>
                    <span class="melt-progress__progress-circle"></span>
                </span>
            </span>
            <span class="melt-progress__tip-right">
                <If cond=MaybeSignal::derive(cx, move || !right_tip.get().is_empty())>
                    <Then slot>
                        { right_tip.get() }
                    </Then>
                </If>
            </span>
         </div>
    }
}
