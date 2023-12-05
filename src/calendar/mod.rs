mod theme;

use crate::{use_theme, utils::mount_style, Theme};
use leptos::*;
use std::ops::Deref;
pub use theme::CalendarTheme;
pub use time::Date;
pub use time::OffsetDateTime;

#[component]
pub fn Calendar(#[prop(into)] value: RwSignal<Date>) -> impl IntoView {
    mount_style("calendar", include_str!("./calendar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color-today: {};",
                theme.common.color_primary
            ));
            css_vars.push_str(&format!(
                "--thaw-font-color-other-month: {};",
                theme.calendar.other_month_font_color,
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.calendar.border_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.calendar.background_color_hover
            ));
        });
        css_vars
    });
    let show_date = create_rw_signal(value.get_untracked());
    create_effect(move |_| {
        let selected_date = value.get();
        let show_date_data = show_date.get_untracked();
        if selected_date.year() != show_date_data.year()
            || selected_date.month() != show_date_data.month()
        {
            show_date.set(selected_date);
        }
    });

    let dates = create_memo(move |_| {
        let show_date = show_date.get();
        let show_date_month = show_date.month();
        let mut dates = vec![];

        let mut current_date = show_date;
        let mut current_weekday_number = None::<u8>;
        loop {
            let Some(date) = current_date.previous_day() else {
                break;
            };
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(current_date.weekday().number_days_from_sunday());
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
            let Some(date) = current_date.next_day() else {
                break;
            };
            if date.month() != show_date_month {
                if current_weekday_number.is_none() {
                    current_weekday_number = Some(current_date.weekday().number_days_from_sunday());
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
    view! {
        <div class="thaw-calendar" style=move || css_vars.get()>
            <div class="thaw-calendar__header">
                <span class="thaw-calendar__header-title">

                    {move || {
                        show_date.with(|date| { format!("{} {}", date.month(), date.year()) })
                    }}

                </span>
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
fn CalendarItem(value: RwSignal<Date>, index: usize, date: CalendarItemDate) -> impl IntoView {
    let is_selected = create_memo({
        let date = date.clone();
        move |_| {
            let value_date = value.get();
            value_date.to_calendar_date() == date.to_calendar_date()
        }
    });
    let weekday_str = vec!["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let on_click = {
        let date = date.clone();
        move |_| {
            value.set(date.deref().clone());
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
enum CalendarItemDate {
    Previous(Date),
    Current(Date),
    Next(Date),
}

impl CalendarItemDate {
    fn is_other_month(&self) -> bool {
        match self {
            CalendarItemDate::Previous(_) | CalendarItemDate::Next(_) => true,
            CalendarItemDate::Current(_) => false,
        }
    }

    fn is_today(&self) -> bool {
        let date = self.deref();
        let now_date = OffsetDateTime::now_utc().date();
        now_date.to_calendar_date() == date.to_calendar_date()
    }
}

impl Deref for CalendarItemDate {
    type Target = Date;

    fn deref(&self) -> &Self::Target {
        match self {
            CalendarItemDate::Previous(date)
            | CalendarItemDate::Current(date)
            | CalendarItemDate::Next(date) => date,
        }
    }
}
