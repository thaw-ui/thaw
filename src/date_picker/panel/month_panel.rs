use crate::chrono::{Month, NaiveDate};
use chrono::Datelike;
use leptos::*;

#[component]
pub fn MonthPanel(value: RwSignal<Option<NaiveDate>>) -> impl IntoView {
    view! {
        <div class="thaw-date-picker-month-panel__header">
        </div>
        <div class="thaw-date-picker-month-panel__months">
            {
                (1..13).map(|index| {
                    let month = Month::try_from(index).unwrap();
                    view! {
                        <MonthPanelItem value month/>
                    }
                }).collect_view()
            }
        </div>
    }
}

#[component]
fn MonthPanelItem(value: RwSignal<Option<NaiveDate>>, month: Month) -> impl IntoView {
    let is_selected = create_memo(move |_| {
        value.with(|value_date| {
            value_date.as_ref().map(|date| date.month()) == Some(month.number_from_month())
        })
    });

    view! {
        <div
            class="thaw-date-picker-month-panel__item"
            class=("thaw-date-picker-month-panel__item--selected", move || is_selected.get())
        >
            <div class="thaw-date-picker-month-panel__item-month">
                {month.name()}
            </div>
        </div>
    }
}
