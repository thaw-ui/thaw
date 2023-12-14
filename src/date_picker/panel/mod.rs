mod date_panel;

use crate::{chrono::NaiveDate, use_theme, Theme};
use date_panel::DatePanel;
use leptos::*;

#[component]
pub fn Panel(
    selected_date: RwSignal<Option<NaiveDate>>,
    date_picker_ref: NodeRef<html::Div>,
    close_panel: Callback<Option<NaiveDate>>,
) -> impl IntoView {
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
                theme.date_picker.panel_other_month_font_color,
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.date_picker.panel_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-item-background-color-hover: {};",
                theme.date_picker.panel_date_item_background_color_hover
            ));
            css_vars.push_str(&format!(
                "--thaw-item-border-color: {};",
                theme.date_picker.panel_border_color
            ));
        });
        css_vars
    });

    let panel_ref = create_node_ref::<html::Div>();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let handle = window_event_listener(ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                if panel_ref.get().is_none() {
                    return;
                }
                if current_el == ***panel_ref.get_untracked().unwrap()
                    || current_el == ***date_picker_ref.get_untracked().unwrap()
                {
                    return;
                }
                el = current_el.parent_element();
            }
            close_panel.call(None);
        });
        on_cleanup(move || handle.remove());
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
        _ = date_picker_ref;
        _ = panel_ref;
    }

    view! {
        <div class="thaw-date-picker-panel" style=move || css_vars.get() ref=panel_ref>
            <DatePanel value=selected_date close_panel />
        </div>
    }
}
