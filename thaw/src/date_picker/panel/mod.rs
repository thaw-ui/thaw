mod date_panel;
mod month_panel;
mod year_panel;

use crate::ConfigInjection;
use chrono::NaiveDate;
use date_panel::DatePanel;
use leptos::{html, prelude::*};
use month_panel::MonthPanel;
use thaw_components::CSSTransition;
use thaw_utils::{now_date, ArcOneCallback, ComponentRef};
use year_panel::YearPanel;

#[component]
pub fn Panel(
    selected_date: RwSignal<Option<NaiveDate>>,
    date_picker_ref: NodeRef<html::Div>,
    #[prop(into)] close_panel: ArcOneCallback<Option<NaiveDate>>,
    #[prop(into)] is_show_panel: Signal<bool>,
    #[prop(optional)] comp_ref: ComponentRef<PanelRef>,
) -> impl IntoView {
    let config_provider = ConfigInjection::expect_context();
    let panel_ref = NodeRef::<html::Div>::new();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let close_panel = close_panel.clone();
        let handle = window_event_listener(leptos::ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            loop {
                let Some(current_el) = el else {
                    return;
                };
                if current_el == *body {
                    break;
                };
                if panel_ref.get().is_none() {
                    return;
                }
                let panel_el = panel_ref.get_untracked().unwrap();
                let date_picker_el = date_picker_ref.get_untracked().unwrap();
                if current_el == **panel_el || current_el == **date_picker_el {
                    return;
                }
                el = current_el.parent_element();
            }
            close_panel(None);
        });
        on_cleanup(move || handle.remove());
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        _ = date_picker_ref;
        _ = panel_ref;
    }
    let panel_variant = RwSignal::new(PanelVariant::Date);
    let show_date = RwSignal::new(selected_date.get_untracked().unwrap_or(now_date()));
    comp_ref.load(PanelRef {
        show_date,
        variant: panel_variant,
    });

    view! {
        <CSSTransition
            node_ref=panel_ref
            name="fade-in-scale-up-transition"
            appear=is_show_panel.get_untracked()
            show=is_show_panel
            let:display
        >
            <div
                class="thaw-config-provider thaw-date-picker-panel"
                data-thaw-id=config_provider.id()
                style=move || display.get().unwrap_or_default()
                node_ref=panel_ref
                on:mousedown=|e| e.prevent_default()
            >

                {move || {
                    match panel_variant.get() {
                        PanelVariant::Date => {
                            let close_panel = close_panel.clone();
                            view! {
                                <DatePanel
                                    value=selected_date
                                    show_date
                                    close_panel
                                    panel_variant
                                />
                            }
                                .into_any()
                        }
                        PanelVariant::Month => {
                            view! { <MonthPanel date_panel_show_date=show_date panel_variant /> }
                                .into_any()
                        }
                        PanelVariant::Year => {
                            view! { <YearPanel date_panel_show_date=show_date panel_variant /> }
                                .into_any()
                        }
                    }
                }}

            </div>
        </CSSTransition>
    }
}

#[derive(Clone)]
pub struct PanelRef {
    show_date: RwSignal<NaiveDate>,
    variant: RwSignal<PanelVariant>,
}

impl PanelRef {
    pub fn init_panel(&self, show_date: NaiveDate) {
        self.show_date.set(show_date);
        self.variant.set(PanelVariant::Date);
    }
}

#[derive(Default, Clone)]
pub enum PanelVariant {
    #[default]
    Date,
    Month,
    Year,
}
