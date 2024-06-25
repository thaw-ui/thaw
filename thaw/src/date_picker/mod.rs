mod panel;
use crate::{Icon, Input, InputSuffix, SignalWatch};
use chrono::NaiveDate;
use leptos::*;
use panel::{Panel, PanelRef};
use thaw_components::{Binder, Follower, FollowerPlacement};
use thaw_utils::{mount_style, now_date, ComponentRef, Model, OptionalProp};

#[component]
pub fn DatePicker(
    #[prop(optional, into)] value: Model<Option<NaiveDate>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
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
    let panel_ref = ComponentRef::<PanelRef>::default();
    let panel_selected_date = create_rw_signal(None::<NaiveDate>);
    _ = panel_selected_date.watch(move |date| {
        let text = date.as_ref().map_or(String::new(), |date| {
            date.format(show_date_format).to_string()
        });
        show_date_text.set(text);
    });

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
        if let Some(panel_ref) = panel_ref.get_untracked() {
            panel_ref.init_panel(value.get_untracked().unwrap_or(now_date()));
        }
        is_show_panel.set(true);
    });

    view! {
        <Binder target_ref=date_picker_ref>
            <div ref=date_picker_ref>
                <Input attrs class value=show_date_text on_focus=open_panel on_blur=on_input_blur>
                    <InputSuffix slot>
                        <Icon icon=icondata_ai::AiCalendarOutlined style="font-size: 18px"/>
                    </InputSuffix>
                </Input>
            </div>
            <Follower slot show=is_show_panel placement=FollowerPlacement::BottomStart>
                <Panel
                    date_picker_ref
                    close_panel
                    selected_date=panel_selected_date
                    comp_ref=panel_ref
                    is_show_panel
                />
            </Follower>
        </Binder>
    }
}
