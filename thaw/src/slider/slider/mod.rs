mod types;

pub use types::*;

use crate::{FieldInjection, Rule};
use leptos::{context::Provider, ev, prelude::*};
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style, Model};

#[component]
pub fn Slider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The rules to validate Field.
    #[prop(optional, into)]
    rules: Vec<SliderRule>,
    /// The current value of the controlled Slider.
    #[prop(optional, into)]
    value: Model<f64>,
    /// Min value of the slider.
    #[prop(default = 0f64.into(), into)]
    min: Signal<f64>,
    /// Max value of the slider.
    #[prop(default = 100f64.into(), into)]
    max: Signal<f64>,
    /// The step in which value is incremented.
    #[prop(optional, into)]
    step: MaybeProp<f64>,
    /// Render the Slider in a vertical orientation, smallest value on the bottom.
    #[prop(optional, into)]
    vertical: Signal<bool>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("slider", include_str!("./slider.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    let is_chldren = children.is_some();
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
            validate.run(Some(SliderRuleTrigger::Input));
        }
    };

    let css_vars = move || {
        let max = max.get();
        let min = min.get();
        let mut css_vars = format!(
            "--thaw-slider--progress: {:.2}%;",
            if max == min {
                0.0
            } else {
                (current_value.get() - min) / (max - min) * 100.0
            }
        );

        if vertical.get() {
            css_vars.push_str("--thaw-slider--direction: 0deg;");
        } else {
            css_vars.push_str("--thaw-slider--direction: 90deg;");
        }

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
        <div
            class=class_list![
                "thaw-slider",
                move || format!("thaw-slider--{}", if vertical.get() { "vertical" } else { "horizontal" }),
                class
            ]
            style=css_vars
        >
            <input
                min=move || min.get()
                max=move || max.get()
                step=move || step.get()
                type="range"
                class="thaw-slider__input"
                id=id
                name=name
                on:input=on_input
                value=current_value.get_untracked()
                prop:value=move || current_value.get()
            />
            <div class="thaw-slider__rail"></div>
            <div class="thaw-slider__thumb"></div>
            <OptionComp value=children let:children>
                <Provider value=SliderInjection {
                    max,
                    min,
                    vertical,
                }>
                    <div class="thaw-slider__datalist">{children()}</div>
                </Provider>
            </OptionComp>
        </div>
    }
}
