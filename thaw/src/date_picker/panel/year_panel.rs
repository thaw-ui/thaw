use super::PanelVariant;
use crate::{
    chrono::{Datelike, NaiveDate},
    Button, ButtonSize, ButtonVariant,
};
use leptos::*;

const MAX_YEAR: i32 = (i32::MAX >> 13) / 10 - 1;
const MIN_YEAR: i32 = (i32::MIN >> 13) / 10 + 1;

#[component]
pub fn YearPanel(
    date_panel_show_date: RwSignal<NaiveDate>,
    panel_variant: RwSignal<PanelVariant>,
) -> impl IntoView {
    let show_min_year = create_rw_signal(date_panel_show_date.get_untracked().year() / 10);
    let previous_year_range = move |_| {
        show_min_year.update(|year| {
            if *year > MIN_YEAR {
                *year -= 1;
            }
        });
    };
    let next_year_range = move |_| {
        show_min_year.update(|year| {
            if *year < MAX_YEAR {
                *year += 1;
            }
        });
    };
    view! {
        <div>
            <div class="thaw-date-picker-year-panel__header">
                <Button
                    variant=ButtonVariant::Link
                    size=ButtonSize::Small
                    icon=icondata_ai::AiArrowLeftOutlined
                    on_click=previous_year_range
                />
                <div>
                    {move || {
                        let year = show_min_year.get();
                        format!("{}0 - {}9", year, year)
                    }}

                </div>
                <Button
                    variant=ButtonVariant::Link
                    size=ButtonSize::Small
                    icon=icondata_ai::AiArrowRightOutlined
                    on_click=next_year_range
                />
            </div>
            <div class="thaw-date-picker-year-panel__years">

                {move || {
                    (-1..=10)
                        .map(|index| {
                            let year = show_min_year.get() * 10 + index;
                            let on_click = move |_| {
                                date_panel_show_date
                                    .update(|date| {
                                        *date = date.with_year(year).unwrap();
                                    });
                                panel_variant.set(PanelVariant::Month);
                            };
                            view! { <YearPanelItem date_panel_show_date year on:click=on_click/> }
                        })
                        .collect_view()
                }}

            </div>
        </div>
    }
}

#[component]
fn YearPanelItem(date_panel_show_date: RwSignal<NaiveDate>, year: i32) -> impl IntoView {
    let is_selected = create_memo(move |_| date_panel_show_date.with(|date| date.year() == year));

    view! {
        <div
            class="thaw-date-picker-year-panel__item"
            class=("thaw-date-picker-year-panel__item--selected", move || is_selected.get())
        >
            <div class="thaw-date-picker-year-panel__item-year">{year}</div>
        </div>
    }
}
