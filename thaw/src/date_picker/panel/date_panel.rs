use super::PanelVariant;
use crate::{Button, ButtonAppearance, ButtonSize, CalendarItemDate, LocaleConfig};
use chrono::{Datelike, Days, Months, NaiveDate};
use leptos::{html, prelude::*};
use std::ops::Deref;
use thaw_components::FollowerInjection;
use thaw_utils::{now_date, ArcOneCallback};

#[component]
pub fn DatePanel(
    value: RwSignal<Option<NaiveDate>>,
    show_date: RwSignal<NaiveDate>,
    close_panel: ArcOneCallback<Option<NaiveDate>>,
    panel_variant: RwSignal<PanelVariant>,
) -> impl IntoView {
    let follower = FollowerInjection::expect_context();
    let locale = LocaleConfig::use_locale();
    let panel_ref = NodeRef::<html::Div>::new();
    Effect::new(move || {
        let Some(_) = panel_ref.get() else {
            return;
        };
        follower.refresh_position();
    });
    let dates = Memo::new(move |_| {
        let show_date = show_date.get();
        let show_date_month = show_date.month();
        let mut dates = vec![];

        let first_weekday = locale.get().first_weekday();
        let last_weekday = first_weekday.pred();

        let mut current_date = show_date;
        let mut current_weekday = None;
        loop {
            let date = current_date - Days::new(1);
            if date.month() != show_date_month {
                if current_weekday.is_none() {
                    current_weekday = Some(current_date.weekday());
                }
                let weekday = current_weekday.unwrap();

                if weekday == first_weekday {
                    break;
                }
                current_weekday = Some(weekday.pred());

                dates.push(CalendarItemDate::Previous(date));
            } else {
                dates.push(CalendarItemDate::Current(date));
            }
            current_date = date;
        }
        dates.reverse();
        dates.push(CalendarItemDate::Current(show_date));
        current_date = show_date;
        current_weekday = None;
        loop {
            let date = current_date + Days::new(1);
            if date.month() != show_date_month {
                if current_weekday.is_none() {
                    current_weekday = Some(current_date.weekday());
                }
                let weekday = current_weekday.unwrap();
                if weekday == last_weekday {
                    break;
                }
                current_weekday = Some(weekday.succ());
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
    let now = {
        let close_panel = close_panel.clone();
        move |_| {
            close_panel(Some(now_date()));
        }
    };
    view! {
        <div class="thaw-date-picker-date-panel" node_ref=panel_ref>
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
                            {move || locale.get().month(show_date.get().month() as u8)}
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
                    {move|| {
                        let first_weekday_number = locale.get().first_weekday().num_days_from_sunday() as u8;
                        let last_weekday_number = first_weekday_number + 6;
                        (first_weekday_number..=last_weekday_number)
                            .into_iter()
                            .map(|n| {
                                view! { <span>{locale.get().ab_day(n%7)}</span>}
                            })
                            .collect_view()
                        }
                    }
                </div>
                <div class="thaw-date-picker-date-panel__dates">
                    {move || {
                        dates
                            .get()
                            .into_iter()
                            .map(|date| {
                                let on_click = {
                                    let date = date.clone();
                                    let close_panel = close_panel.clone();
                                    move |_| {
                                        close_panel(Some(*date.deref()));
                                    }
                                };
                                view! { <DatePanelItem value date=date on:click=on_click /> }
                            })
                            .collect_view()
                    }}

                </div>
            </div>
            <div class="thaw-date-picker-date-panel__footer">
                <Button size=ButtonSize::Small on_click=now>
                    { move || locale.get().today() }
                </Button>
            </div>
        </div>
    }
}

#[component]
fn DatePanelItem(value: RwSignal<Option<NaiveDate>>, date: CalendarItemDate) -> impl IntoView {
    let is_selected = Memo::new({
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
