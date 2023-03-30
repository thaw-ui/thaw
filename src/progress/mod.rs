use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

#[component]
pub fn Progress(
    cx: Scope,
    #[prop(optional)] left_tip: Option<ReadSignal<String>>,
    #[prop(optional)] right_tip: Option<ReadSignal<String>>,
    percentage: ReadSignal<f64>,
) -> impl IntoView {
    let class_name = mount_style("progress", || style_sheet_str!("./src/progress/progress.css"));
    let style = move || format!("width: {}%", percentage.get());
    view! {
        cx, class=class_name,
        <div class="melt-progress">
            <span class="melt-progress__tip-left">
                {
                    move || {
                        if let Some(left_tip) = left_tip {
                            left_tip.get()
                        }  else {
                            "".into()
                        }
                    }
                }
            </span>
            <span class="melt-progress__progress">
                <span class="melt-progress__progress-inner" style=style>
                    <span class="melt-progress__progress-circle"></span>
                </span>
            </span>
            <span class="melt-progress__tip-right">
                {
                    move || {
                        if let Some(right_tip) = right_tip {
                            right_tip.get()
                        }  else {
                            "".into()
                        }
                    }
                }
            </span>
         </div>
    }
}
