use super::PanelVariant;
use crate::{Button, ButtonSize, ButtonAppearance, CalendarItemDate};
use chrono::{Datelike, Days, Month, Months, NaiveDate};
use leptos::*;
use std::ops::Deref;
use thaw_utils::now_date;

#[component]
pub fn DatePanel(
    value: RwSignal<Option<NaiveDate>>,
    show_date: RwSignal<NaiveDate>,
    close_panel: Callback<Option<NaiveDate>>,
    panel_variant: RwSignal<PanelVariant>,
) -> impl IntoView {
    let dates = create_memo(move |_| {
        let show_date = show_date.get();
        let show_date_month = show_date.month();
        let mut dates = vec![];

        let mut current_date = show_date;
        let mut current_weekday_number = None::<u32>;
        loop {
            let date = current_date - Days::new(1);
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(current_date.weekday().num_days_from_sunday());
                }
                let weekday_number = current_weekday_number.unwrap();
                if weekday_number == 0 {
                    break;
                }
                current_weekday_number = Some(weekday_number - 1);

                dates.push(CalendarItemDate::Previous(date));
            } else {
                dates.push(CalendarItemDate::Current(date));
            }
            current_date = date;
        }
        dates.reverse();
        dates.push(CalendarItemDate::Current(show_date));
        current_date = show_date;
        current_weekday_number = None;
        loop {
            let date = current_date + Days::new(1);
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(current_date.weekday().num_days_from_sunday());
                }
                let weekday_number = current_weekday_number.unwrap();
                if weekday_number == 6 {
                    break;
                }
                current_weekday_number = Some(weekday_number + 1);
                dates.push(CalendarItemDate::Next(date));
            } else {
                dates.push(CalendarItemDate::Current(date));
            }
            current_date = date;
        }
        dates
    });
    let previous_year = move |_| {
        show_date.update(|date| {
            *date = *date - Months::new(12);
        });
    };
    let next_year = move |_| {
        show_date.update(|date| {
            *date = *date + Months::new(12);
        });
    };
    let previous_month = move |_| {
        show_date.update(|date| {
            *date = *date - Months::new(1);
        });
    };
    let next_month = move |_| {
        show_date.update(|date| {
            *date = *date + Months::new(1);
        });
    };
    let now = Callback::new(move |_| {
        close_panel.call(Some(now_date()));
    });
    view! {
        <div>
            <div class="thaw-date-picker-date-panel__calendar">
                <div class="thaw-date-picker-date-panel__header">
                    <Button
                        appearance=ButtonAppearance::Transparent
                        size=ButtonSize::Small
                        icon=icondata_ai::AiArrowLeftOutlined
                        on_click=previous_year
                    />
                    <Button
                        appearance=ButtonAppearance::Transparent
                        size=ButtonSize::Small
                        icon=icondata_ai::AiLeftOutlined
                        on_click=previous_month
                    />
                    <div class="thaw-date-picker-date-panel__header-month-year">
                        <Button
                            appearance=ButtonAppearance::Subtle
                            size=ButtonSize::Small
                            on_click=move |_| panel_variant.set(PanelVariant::Month)
                        >
                            {move || Month::try_from(show_date.get().month() as u8).unwrap().name()}
                        </Button>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            size=ButtonSize::Small
                            on_click=move |_| panel_variant.set(PanelVariant::Year)
                        >
                            {move || show_date.get().year()}
                        </Button>
                    </div>
                    <Button
                        appearance=ButtonAppearance::Transparent
                        size=ButtonSize::Small
                        icon=icondata_ai::AiRightOutlined
                        on_click=next_month
                    />
                    <Button
                        appearance=ButtonAppearance::Transparent
                        size=ButtonSize::Small
                        icon=icondata_ai::AiArrowRightOutlined
                        on_click=next_year
                    />
                </div>
                <div class="thaw-date-picker-date-panel__weekdays">
                    <span>"Su"</span>
                    <span>"Mo"</span>
                    <span>"Tu"</span>
                    <span>"We"</span>
                    <span>"Th"</span>
                    <span>"Fr"</span>
                    <span>"Sa"</span>
                </div>
                <div class="thaw-date-picker-date-panel__dates">
                    {move || {
                        dates
                            .get()
                            .into_iter()
                            .map(|date| {
                                let on_click = {
                                    let date = date.clone();
                                    move |_| {
                                        close_panel.call(Some(*date.deref()));
                                    }
                                };
                                view! { <DatePanelItem value date=date on:click=on_click/> }
                            })
                            .collect_view()
                    }}

                </div>
            </div>
            <div class="thaw-date-picker-date-panel__footer">
                <Button size=ButtonSize::Tiny on_click=now>
                    "Now"
                </Button>
            </div>
        </div>
    }
}

#[component]
fn DatePanelItem(value: RwSignal<Option<NaiveDate>>, date: CalendarItemDate) -> impl IntoView {
    let is_selected = create_memo({
        let date = date.clone();
        move |_| value.with(|value_date| value_date.as_ref() == Some(date.deref()))
    });

    view! {
        <div
            class="thaw-date-picker-date-panel__item"
            class=("thaw-date-picker-date-panel__item--other-month", date.is_other_month())
            class=("thaw-date-picker-date-panel__item--selected", move || is_selected.get())
        >
            <div class="thaw-date-picker-date-panel__item-day">
                {date.day()}
                {if date.is_today() {
                    view! { <div class="thaw-date-picker-date-panel__item-sup"></div> }.into()
                } else {
                    None
                }}

            </div>
        </div>
    }
}
