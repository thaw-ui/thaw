mod slider_label;

pub use slider_label::SliderLabel;

use leptos::{context::Provider, ev, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Slider(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The current value of the controlled Slider.
    #[prop(optional, into)]
    value: Model<f64>,
    /// Min value of the slider.
    #[prop(default = 0f64.into(), into)]
    min: MaybeSignal<f64>,
    /// Max value of the slider.
    #[prop(default = 100f64.into(), into)]
    max: MaybeSignal<f64>,
    /// The step in which value is incremented.
    #[prop(optional, into)]
    step: MaybeProp<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("slider", include_str!("./slider.css"));

    let is_chldren = children.is_some();
    let list_id = is_chldren.then(|| uuid::Uuid::new_v4().to_string());
    let current_value = Memo::new(move |_| {
        let max = max.get();
        let min = min.get();
        let v = value.get();
        if v > max {
            max
        } else if v < min {
            min
        } else {
            v
        }
    });

    let on_input = move |e: ev::Event| {
        if let Ok(range_value) = event_target_value(&e).parse::<f64>() {
            value.set(range_value);
        }
    };

    let css_vars = move || {
        let max = max.get();
        let min = min.get();
        let mut css_vars = format!(
            "--thaw-slider--direction: 90deg;--thaw-slider--progress: {:.2}%;",
            if max == min {
                0.0
            } else {
                (current_value.get() - min) / (max - min) * 100.0
            }
        );

        if is_chldren {
            css_vars.push_str(&format!("--thaw-slider--max: {:.2};", max));
            css_vars.push_str(&format!("--thaw-slider--min: {:.2};", min));
        }

        if let Some(step) = step.get() {
            if step > 0.0 {
                css_vars.push_str(&format!(
                    "--thaw-slider--steps-percent: {:.2}%",
                    step * 100.0 / (max - min)
                ));
            }
        }
        css_vars
    };

    view! {
        <Provider value=SliderInjection { max, min }>
            <div class=class_list!["thaw-slider", class] style=css_vars>
                <input
                    min=move || min.get()
                    max=move || max.get()
                    step=move || step.get()
                    type="range"
                    class="thaw-slider__input"
                    on:input=on_input
                    value=current_value.get_untracked()
                    prop:value=move || current_value.get()
                    list=list_id.clone()
                />
                <div class="thaw-slider__rail"></div>
                <div class="thaw-slider__thumb"></div>
                <OptionComp value=children let:children>
                    <datalist id=list_id class="thaw-slider__datalist">
                        {children()}
                    </datalist>
                </OptionComp>
            </div>
        </Provider>
    }
}

#[derive(Clone)]
pub(crate) struct SliderInjection {
    pub max: MaybeSignal<f64>,
    pub min: MaybeSignal<f64>,
}

impl SliderInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
