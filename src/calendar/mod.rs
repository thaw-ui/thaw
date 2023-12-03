use crate::utils::mount_style;
use leptos::*;
use time::Date;
pub use time::OffsetDateTime;

#[component]
pub fn Calendar(value: RwSignal<OffsetDateTime>) -> impl IntoView {
    mount_style("calendar", include_str!("./calendar.css"));
    let dates = create_memo(move |_| {
        let date = value.get().date();
        let weekday_number = date.weekday().number_days_from_sunday();
        let mut dates = vec![];

        let mut current_date = date;
        for _ in 0..weekday_number {
            if let Some(date) = current_date.previous_day() {
                dates.push(date);
                current_date = date
            }
        }
        dates.reverse();

        let month = date.month();
        let mut current_date = date;
        let mut date_count = weekday_number + 1;
        dates.push(current_date);
        loop {
            let Some(date) = current_date.next_day() else {
                break;
            };
            if date.month() != month {
                if date_count % 7 == 0 {
                    break;
                }
            }
            dates.push(date);
            current_date = date;
            date_count += 1;
        }
        dates
    });
    view! {
        <div class="thaw-calendar">
            <div class="thaw-calendar__header">
                <span class="thaw-calendar__header-title">

                    {move || {
                        value
                            .with(|date_time| {
                                let date = date_time.date();
                                format!("{} {}", date.month(), date.year())
                            })
                    }}

                </span>
            </div>
            <div class="thaw-calendar__dates">
                <For each=move || dates.get() key=|date| date.to_calendar_date() let:date>
                    <CalendarItem date/>
                </For>
            </div>
        </div>
    }
}

#[component]
pub fn CalendarItem(date: Date) -> impl IntoView {
    view! {
        <div class="thaw-calendar__dates-item">
            <div class="thaw-calendar__dates-item__header">{date.day()}</div>
        </div>
    }
}
