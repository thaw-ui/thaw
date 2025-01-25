use crate::RatingInjection;

use super::{rating_item::RatingItem, RatingColor, RatingSize};
use leptos::{context::Provider, prelude::*, reactive::wrappers::write::SignalSetter};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn RatingDisplay(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The value of the rating.
    #[prop(optional, into)]
    value: Signal<f32>,
    /// The max value of the rating. This controls the number of rating items displayed.
    /// Must be a whole number greater than 1.
    #[prop(default = 5.into(), into)]
    max: Signal<u8>,
    /// Sets the size of the Rating items.
    #[prop(default = RatingSize::Medium.into(), into)]
    size: Signal<RatingSize>,
    /// Rating color.
    #[prop(optional, into)]
    color: Signal<RatingColor>,
) -> impl IntoView {
    mount_style("rating", include_str!("../rating/rating.css"));

    view! {
        <div
            role="img"
            class=class_list![
                "thaw-rating-display",
                move || format!("thaw-rating-display--{}", size.get().as_str()),
                class
            ]
        >
            <Provider value=RatingInjection {
                value: (value, SignalSetter::default()).into(),
                hovered_value: RwSignal::new(None::<f32>),
                name: StoredValue::new(String::new()),
                step: 0.5.into(),
                size,
                color,
                interactive: false,
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
            <span aria-hidden="true" class="thaw-rating-display__value-text">
                {move || value.get()}
            </span>
        </div>
    }
}
