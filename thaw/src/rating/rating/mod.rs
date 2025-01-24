mod types;

pub(crate) use types::*;

use super::rating_item::RatingItem;
use leptos::{context::Provider, prelude::*};
use thaw_utils::{mount_style, Model};
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement, MouseEvent};

#[component]
pub fn Rating(
    /// The value of the rating
    #[prop(optional, into)]
    value: Model<f32>,
    /// The max value of the rating. This controls the number of rating items displayed. Must be a whole number greater than 1.
    #[prop(default = 5.into(), into)]
    max: Signal<u8>,
) -> impl IntoView {
    mount_style("rating", include_str!("./rating.css"));

    let name = StoredValue::new(uuid::Uuid::new_v4().to_string());
    let hovered_value = RwSignal::new(None::<f32>);

    let on_mouseover = move |e: MouseEvent| {
        if let Some(el) = is_rating_radio_item(e.target().unwrap(), &name.read_value()) {
            if let Ok(new_value) = el.value().parse::<f32>() {
                hovered_value.set(Some(new_value));
            }
        }
    };

    let on_mouseleave = move |_| {
        hovered_value.set(None);
    };

    view! {
        <div role="radiogroup" class="thaw-rating" on:mouseover=on_mouseover on:mouseleave=on_mouseleave>
            <Provider value=RatingInjection { value, hovered_value, name }>
                {move || {
                    let mut max = max.get();
                    if max < 2 {
                        max = 2
                    }

                    (0..max).into_iter().map(|i| {

                        view! {
                            <RatingItem value={i + 1}/>
                        }
                    }).collect_view()
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
