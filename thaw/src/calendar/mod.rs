use crate::{Button, ButtonGroup, LocaleConfig};
use chrono::{Datelike, Days, Local, Months, NaiveDate};
use leptos::{prelude::*, tachys::view::any_view::AnyView};
use std::{ops::Deref, sync::Arc};
use thaw_utils::{class_list, mount_style, OptionModel, OptionModelWithValue};

#[component]
pub fn Calendar(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// selected date.
    #[prop(optional, into)]
    value: OptionModel<NaiveDate>,
    #[prop(optional, into)] children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    mount_style("calendar", include_str!("./calendar.css"));

    let locale = LocaleConfig::use_locale();

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
        <div class=class_list!["thaw-calendar", class]>
            <div class="thaw-calendar__header">
                <span class="thaw-calendar__header-title">

                    {move || {
                        show_date
                            .with(|date| {
                                format!(
                                    "{} {}",
                                    locale.get().month(date.month() as u8),
                                    date.year(),
                                )
                            })
                    }}

                </span>
                <ButtonGroup>
                    <Button icon=icondata_ai::AiLeftOutlined on_click=previous_month />
                    <Button on_click=today>{move || locale.get().today()}</Button>
                    <Button icon=icondata_ai::AiRightOutlined on_click=next_month />
                </ButtonGroup>
            </div>
            <div class="thaw-calendar__dates">

                {move || {
                    dates
                        .get()
                        .into_iter()
                        .enumerate()
                        .map(|(index, date)| {
                            view! {
                                <CalendarItem
                                    value
                                    index=index
                                    date=date
                                    children=children.clone()
                                />
                            }
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
    children: Option<CalendarChildrenFn>,
) -> impl IntoView {
    let locale = LocaleConfig::use_locale();
    let is_selected = Memo::new({
        let date = date.clone();
        move |_| {
            value.with(|value_date| match value_date {
                OptionModelWithValue::T(v) => v == date.deref(),
                OptionModelWithValue::Option(v) => v.as_ref() == Some(date.deref()),
            })
        }
    });
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
                        <span class="thaw-calendar-item__header-title">
                            {
                                let date = date.clone();
                                move || locale.get().ab_weekday(date.weekday())
                            }
                        </span>
                    }
                        .into()
                } else {
                    None
                }}

            </div>
            {children.map(|c| c(date.deref()))}
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

#[derive(Clone)]
pub struct CalendarChildrenFn(Arc<dyn Fn(&NaiveDate) -> AnyView + Send + Sync>);

impl Deref for CalendarChildrenFn {
    type Target = Arc<dyn Fn(&NaiveDate) -> AnyView + Send + Sync>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F, C> From<F> for CalendarChildrenFn
where
    F: Fn(&NaiveDate) -> C + Send + Sync + 'static,
    C: RenderHtml + Send + 'static,
{
    fn from(f: F) -> Self {
        Self(Arc::new(move |date| f(date).into_any()))
    }
}
