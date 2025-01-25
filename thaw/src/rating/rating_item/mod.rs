use crate::RatingInjection;
use leptos::{either::Either, prelude::*};
use thaw_components::{If, Then};
use thaw_utils::class_list;

#[component]
pub fn RatingItem(value: u8) -> impl IntoView {
    let rating = RatingInjection::expect_context();

    let icon_fill_width = Memo::new(move |_| {
        let displayed_rating_value = rating
            .hovered_value
            .get()
            .unwrap_or_else(|| (rating.value.get().unwrap_or_default() * 2.0).round() / 2.0);
        let value = f32::from(value);

        if displayed_rating_value >= value {
            1.0
        } else if displayed_rating_value >= value - 0.5 {
            0.5
        } else {
            0.0
        }
    });

    let half = Signal::derive(move || rating.step.get() == 0.5);

    view! {
        <span class=class_list![
            "thaw-rating-item",
            ("thaw-rating-item--filled", !rating.interactive),
            ("thaw-rating-item--half", move || half.get() && icon_fill_width.get() == 0.5),
            move || format!("thaw-rating-item--{}", rating.color.get().as_str()),
            move || format!("thaw-rating-item--{}", rating.size.get().as_str())
        ]>
            {if rating.interactive {
                Either::Left(
                    view! {
                        <If cond=half>
                            <Then slot>
                                <input
                                    type="radio"
                                    name=rating.name
                                    aria-label=f32::from(value) - 0.5
                                    class="thaw-rating-item__half-value-input"
                                    value=f32::from(value) - 0.5
                                />
                            </Then>
                        </If>
                        <input
                            type="radio"
                            name=rating.name
                            aria-label=value
                            class="thaw-rating-item__full-value-input"
                            value=value
                        />
                    },
                )
            } else {
                Either::Right(())
            }} <If cond=Signal::derive(move || icon_fill_width.get() < 1.0)>
                <Then slot>
                    <div aria-hidden="true" class="thaw-rating-item__unselected-icon">
                        {if rating.interactive {
                            Either::Left(
                                view! {
                                    <svg
                                        fill="currentColor"
                                        aria-hidden="true"
                                        width="1em"
                                        height="1em"
                                        viewBox="0 0 20 20"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                            d="M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Zm.9.44L8.07 7.25a1 1 0 0 1-.75.55L3 8.43l3.12 3.04a1 1 0 0 1 .3.89l-.75 4.3 3.87-2.03a1 1 0 0 1 .93 0l3.86 2.03-.74-4.3a1 1 0 0 1 .29-.89L17 8.43l-4.32-.63a1 1 0 0 1-.75-.55L10 3.35Z"
                                            fill="currentColor"
                                        ></path>
                                    </svg>
                                },
                            )
                        } else {
                            Either::Right(
                                view! {
                                    <svg
                                        fill="currentColor"
                                        aria-hidden="true"
                                        width="1em"
                                        height="1em"
                                        viewBox="0 0 20 20"
                                        xmlns="http://www.w3.org/2000/svg"
                                    >
                                        <path
                                            d="M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Z"
                                            fill="currentColor"
                                        ></path>
                                    </svg>
                                },
                            )
                        }}
                    </div>
                </Then>
            </If> <If cond=Signal::derive(move || icon_fill_width.get() > 0.0)>
                <Then slot>
                    <div aria-hidden="true" class="thaw-rating-item__selected-icon">
                        <svg
                            fill="currentColor"
                            aria-hidden="true"
                            width="1em"
                            height="1em"
                            viewBox="0 0 20 20"
                            xmlns="http://www.w3.org/2000/svg"
                        >
                            <path
                                d="M9.1 2.9a1 1 0 0 1 1.8 0l1.93 3.91 4.31.63a1 1 0 0 1 .56 1.7l-3.12 3.05.73 4.3a1 1 0 0 1-1.45 1.05L10 15.51l-3.86 2.03a1 1 0 0 1-1.45-1.05l.74-4.3L2.3 9.14a1 1 0 0 1 .56-1.7l4.31-.63L9.1 2.9Z"
                                fill="currentColor"
                            ></path>
                        </svg>
                    </div>
                </Then>
            </If>
        </span>
    }
}
