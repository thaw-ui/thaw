use super::PanelVariant;
use crate::{
    chrono::{Datelike, Month, Months, NaiveDate},
    Button, ButtonSize, ButtonVariant,
};
use leptos::*;

#[component]
pub fn MonthPanel(
    date_panel_show_date: RwSignal<NaiveDate>,
    panel_variant: RwSignal<PanelVariant>,
) -> impl IntoView {
    let show_date = create_rw_signal(date_panel_show_date.get_untracked());
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
    view! {
        <div class="thaw-date-picker-month-panel">
            <div class="thaw-date-picker-month-panel__header">
                <Button
                    variant=ButtonVariant::Link
                    size=ButtonSize::Small
                    icon=icondata_ai::AiArrowLeftOutlined
                    on_click=previous_year
                />
                <div class="thaw-date-picker-date-panel__header-year">
                    <Button
                        variant=ButtonVariant::Text
                        size=ButtonSize::Small
                        on_click=move |_| panel_variant.set(PanelVariant::Year)
                    >
                        {move || show_date.get().year()}
                    </Button>
                </div>
                <Button
                    variant=ButtonVariant::Link
                    size=ButtonSize::Small
                    icon=icondata_ai::AiArrowRightOutlined
                    on_click=next_year
                />
            </div>
            <div class="thaw-date-picker-month-panel__months">

                {(1..=12)
                    .map(|index| {
                        let month = Month::try_from(index).unwrap();
                        let on_click = move |_| {
                            date_panel_show_date
                                .update(|date| {
                                    let show_date = show_date.get_untracked();
                                    *date = date
                                        .with_month(index.into())
                                        .unwrap()
                                        .with_year(show_date.year())
                                        .unwrap();
                                });
                            panel_variant.set(PanelVariant::Date);
                        };
                        view! { <MonthPanelItem date_panel_show_date month on:click=on_click/> }
                    })
                    .collect_view()}

            </div>
        </div>
    }
}

#[component]
fn MonthPanelItem(date_panel_show_date: RwSignal<NaiveDate>, month: Month) -> impl IntoView {
    let is_selected = create_memo(move |_| {
        date_panel_show_date.with(|date| date.month() == month.number_from_month())
    });

    view! {
        <div
            class="thaw-date-picker-month-panel__item"
            class=("thaw-date-picker-month-panel__item--selected", move || is_selected.get())
        >
            <div class="thaw-date-picker-month-panel__item-month">{month.name().split_at(3).0}</div>
        </div>
    }
}
