mod types;

pub use types::*;

use super::rating_item::RatingItem;
use crate::{FieldInjection, Rule};
use leptos::{context::Provider, prelude::*};
use thaw_utils::{class_list, OptionModel};
use wasm_bindgen::JsCast;
use web_sys::{Event, EventTarget, HtmlInputElement, MouseEvent};

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/rating/rating/rating.css");

#[component]
pub fn Rating(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    #[prop(optional, into)] rules: Vec<RatingRule>,
    /// Name for the Radio inputs. If not provided, one will be automatically generated.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The value of the rating.
    #[prop(optional, into)]
    value: OptionModel<f32>,
    /// The max value of the rating. This controls the number of rating items displayed.
    /// Must be a whole number greater than 1.
    #[prop(default = 5.into(), into)]
    max: Signal<u8>,
    /// Sets the precision to allow half-filled shapes in Rating.
    #[prop(default = 1.0.into(), into)]
    step: Signal<f32>,
    /// Sets the size of the Rating items.
    #[prop(default = RatingSize::ExtraLarge.into(), into)]
    size: Signal<RatingSize>,
    /// Rating color.
    #[prop(optional, into)]
    color: Signal<RatingColor>,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("rating", include_str!("./rating.css"));

    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);

    let name = Memo::new(move |_| {
        name.get()
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    });
    let hovered_value = RwSignal::new(None::<f32>);

    let on_change = move |e: Event| {
        if let Some(el) = is_rating_radio_item(e.target().unwrap(), &name.read()) {
            if let Ok(new_value) = el.value().parse::<f32>() {
                value.set(Some(new_value));
                validate.run(Some(RatingRuleTrigger::Change));
            }
        }
    };

    let on_mouseover = move |e: MouseEvent| {
        if let Some(el) = is_rating_radio_item(e.target().unwrap(), &name.read()) {
            if let Ok(new_value) = el.value().parse::<f32>() {
                hovered_value.set(Some(new_value));
            }
        }
    };

    let on_mouseleave = move |_| {
        hovered_value.set(None);
    };

    view! {
        <div
            role="radiogroup"
            class=class_list!["thaw-rating", class]
            id=id
            on:change=on_change
            on:mouseover=on_mouseover
            on:mouseleave=on_mouseleave
        >
            <Provider value=RatingInjection {
                value,
                hovered_value,
                name,
                step,
                size,
                color,
                interactive: true,
            }>
                {move || {
                    let mut max = max.get();
                    if max < 2 {
                        max = 2;
                    }
                    (0..max)
                        .into_iter()
                        .map(|i| {
                            view! { <RatingItem value=i + 1 /> }
                        })
                        .collect_view()
                }}
            </Provider>
        </div>
    }
}

fn is_rating_radio_item(target: EventTarget, name: &String) -> Option<HtmlInputElement> {
    target
        .dyn_into::<HtmlInputElement>()
        .ok()
        .filter(|el| el.type_() == "radio" && &el.name() == name)
}
