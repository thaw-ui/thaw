use super::PanelVariant;
use crate::{Button, ButtonAppearance, ButtonSize};
use chrono::{Datelike, Month, Months, NaiveDate};
use leptos::{html, prelude::*};
use thaw_components::FollowerInjection;

#[component]
pub fn MonthPanel(
    date_panel_show_date: RwSignal<NaiveDate>,
    panel_variant: RwSignal<PanelVariant>,
) -> impl IntoView {
    let follower = FollowerInjection::expect_context();
    let panel_ref = NodeRef::<html::Div>::new();
    Effect::new(move || {
        let Some(_) = panel_ref.get() else {
            return;
        };
        follower.refresh_position();
    });

    let show_date = RwSignal::new(date_panel_show_date.get_untracked());
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
        <div class="thaw-date-picker-month-panel" node_ref=panel_ref>
            <div class="thaw-date-picker-month-panel__header">
                <Button
                    appearance=ButtonAppearance::Transparent
                    size=ButtonSize::Small
                    icon=icondata_ai::AiArrowLeftOutlined
                    on_click=previous_year
                />
                <div class="thaw-date-picker-date-panel__header-year">
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
                        view! { <MonthPanelItem date_panel_show_date month on:click=on_click /> }
                    })
                    .collect_view()}

            </div>
        </div>
    }
}

#[component]
fn MonthPanelItem(date_panel_show_date: RwSignal<NaiveDate>, month: Month) -> impl IntoView {
    let is_selected = Memo::new(move |_| {
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
