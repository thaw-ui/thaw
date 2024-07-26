use crate::{Button, ButtonGroup};
use chrono::{Datelike, Days, Local, Month, Months, NaiveDate};
use leptos::prelude::*;
use std::ops::Deref;
use thaw_utils::{class_list, mount_style, OptionModel};

#[component]
pub fn Calendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Default selected date.
    #[prop(optional, into)] value: OptionModel<NaiveDate>,
) -> impl IntoView {
    mount_style("calendar", include_str!("./calendar.css"));
    let show_date = RwSignal::new(value.get_untracked().unwrap_or(now_date()));
    Effect::new(move |_| {
        if let Some(selected_date) = value.get() {
            let show_date_data = show_date.get_untracked();
            if selected_date.year() != show_date_data.year()
                || selected_date.month() != show_date_data.month()
            {
                show_date.set(selected_date);
            }
        }
    });

    let dates = Memo::new(move |_| {
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

    let previous_month = move |_| {
        show_date.update(|date| {
            *date = *date - Months::new(1);
        });
    };
    let today = move |_| {
        show_date.set(Local::now().date_naive());
    };
    let next_month = move |_| {
        show_date.update(|date| {
            *date = *date + Months::new(1);
        });
    };

    view! {
        <div
            class=class_list!["thaw-calendar", class]
        >
            <div class="thaw-calendar__header">
                <span class="thaw-calendar__header-title">

                    {move || {
                        show_date
                            .with(|date| {
                                format!(
                                    "{} {}",
                                    Month::try_from(date.month() as u8).unwrap().name(),
                                    date.year(),
                                )
                            })
                    }}

                </span>
                <ButtonGroup>
                    <Button
                        icon=icondata_ai::AiLeftOutlined
                        on_click=previous_month
                    />
                    <Button on_click=today>
                        "Today"
                    </Button>
                    <Button
                        icon=icondata_ai::AiRightOutlined
                        on_click=next_month
                    />
                </ButtonGroup>
            </div>
            <div class="thaw-calendar__dates">

                {move || {
                    dates
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, date)| {
                            view! { <CalendarItem value index=index date=date/> }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
fn CalendarItem(
    value: OptionModel<NaiveDate>,
    index: usize,
    date: CalendarItemDate,
) -> impl IntoView {
    let is_selected = Memo::new({
        let date = date.clone();
        move |_| value.with(|value_date| value_date == Some(date.deref()))
    });
    let weekday_str = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let on_click = {
        let date = date.clone();
        move |_| {
            value.set(Some(*date.deref()));
        }
    };
    view! {
        <div
            class="thaw-calendar-item"
            class=("thaw-calendar-item--other-month", date.is_other_month())
            class=("thaw-calendar-item--today", date.is_today())
            class=("thaw-calendar-item--selected", move || is_selected.get())
            on:click=on_click
        >
            <div class="thaw-calendar-item__header">
                <span class="thaw-calendar-item__header-day">{date.day()}</span>

                {if index < 7 {
                    view! {
                        <span class="thaw-calendar-item__header-title">{weekday_str[index]}</span>
                    }
                        .into()
                } else {
                    None
                }}

            </div>
            <div class="thaw-calendar-item__bar"></div>
        </div>
    }
}

#[derive(Clone, PartialEq)]
pub(crate) enum CalendarItemDate {
    Previous(NaiveDate),
    Current(NaiveDate),
    Next(NaiveDate),
}

impl CalendarItemDate {
    pub fn is_other_month(&self) -> bool {
        match self {
            CalendarItemDate::Previous(_) | CalendarItemDate::Next(_) => true,
            CalendarItemDate::Current(_) => false,
        }
    }

    pub fn is_today(&self) -> bool {
        let date = self.deref();
        let now_date = now_date();
        &now_date == date
    }
}

impl Deref for CalendarItemDate {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        match self {
            CalendarItemDate::Previous(date)
            | CalendarItemDate::Current(date)
            | CalendarItemDate::Next(date) => date,
        }
    }
}

pub(crate) fn now_date() -> NaiveDate {
    Local::now().date_naive()
}
