mod theme;

use crate::{
    chrono::NaiveDate,
    components::{Binder, Follower, FollowerPlacement},
    use_theme,
    utils::mount_style,
    AiIcon, Icon, Input, InputSuffix, Theme,
};
use leptos::*;
pub use theme::DatePickerTheme;

#[component]
pub fn DatePicker(#[prop(optional, into)] value: RwSignal<Option<NaiveDate>>) -> impl IntoView {
    mount_style("date-picker", include_str!("./date-picker.css"));
    let date_picker_ref = create_node_ref::<html::Div>();
    let is_show_panel = create_rw_signal(false);
    let show_date_text = create_rw_signal(String::new());
    let show_date_format = "%Y-%m-%d";
    let update_show_date_text = move || {
        value.with_untracked(move |date| {
            let text = date.as_ref().map_or(String::new(), |date| {
                date.format(show_date_format).to_string()
            });
            show_date_text.set(text);
        });
    };
    update_show_date_text();
    let panel_selected_date = create_rw_signal(None::<NaiveDate>);

    let on_input_blur = Callback::new(move |_| {
        if let Ok(date) =
            NaiveDate::parse_from_str(&show_date_text.get_untracked(), show_date_format)
        {
            if value.get_untracked() != Some(date) {
                value.set(Some(date));
                update_show_date_text();
            }
        } else {
            update_show_date_text();
        }
    });

    let close_panel = Callback::new(move |date: Option<NaiveDate>| {
        if value.get_untracked() != date {
            if date.is_some() {
                value.set(date);
            }
            update_show_date_text();
        }
        is_show_panel.set(false);
    });

    let open_panel = Callback::new(move |_| {
        panel_selected_date.set(value.get_untracked());
        is_show_panel.set(true);
    });

    view! {
        <Binder target_ref=date_picker_ref>
            <div ref=date_picker_ref>
                <Input value=show_date_text on_focus=open_panel on_blur=on_input_blur>
                    <InputSuffix slot>
                        <Icon
                            icon=Icon::from(AiIcon::AiCalendarOutlined)
                            style="font-size: 18px"
                        />
                    </InputSuffix>
                </Input>
            </div>
            <Follower slot show=is_show_panel placement=FollowerPlacement::BottomStart>
                <Panel date_picker_ref close_panel/>
            </Follower>
        </Binder>
    }
}

#[component]
fn Panel(
    date_picker_ref: NodeRef<html::Div>,
    close_panel: Callback<Option<NaiveDate>>,
) -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-item-font-color: {};",
                theme.common.color_primary
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
        </div>
    }
}
