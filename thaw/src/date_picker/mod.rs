mod panel;

use crate::{FieldInjection, FieldValidationState, Icon, Input, InputSuffix, Rule};
use chrono::NaiveDate;
use leptos::{html, prelude::*};
use panel::{Panel, PanelRef};
use std::ops::Deref;
use thaw_components::{Binder, Follower, FollowerPlacement};
use thaw_utils::{
    class_list, mount_style, now_date, ComponentRef, OptionModel, OptionModelWithValue, SignalWatch,
};

#[component]
pub fn DatePicker(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] id: MaybeProp<String>,
    /// A string specifying a name for the input control.
    /// This name is submitted along with the control's value when the form data is submitted.
    #[prop(optional, into)]
    name: MaybeProp<String>,
    /// The rules to validate Field.
    #[prop(optional, into)]
    rules: Vec<DatePickerRule>,
    /// Set the date picker value.
    #[prop(optional, into)]
    value: OptionModel<NaiveDate>,
) -> impl IntoView {
    mount_style("date-picker", include_str!("./date-picker.css"));
    let (id, name) = FieldInjection::use_id_and_name(id, name);
    let validate = Rule::validate(rules, value, name);
    let date_picker_ref = NodeRef::<html::Div>::new();
    let is_show_panel = RwSignal::new(false);
    let show_date_text = RwSignal::new(String::new());
    let show_date_format = "%Y-%m-%d";
    let update_show_date_text = move || {
        value.with_untracked(move |date| {
            let text = match date {
                OptionModelWithValue::T(v) => v.format(show_date_format).to_string(),
                OptionModelWithValue::Option(v) => v.map_or(String::new(), |date| {
                    date.format(show_date_format).to_string()
                }),
            };

            show_date_text.set(text);
        });
    };
    update_show_date_text();
    let panel_ref = ComponentRef::<PanelRef>::default();
    let panel_selected_date = RwSignal::new(None::<NaiveDate>);
    _ = panel_selected_date.watch(move |date| {
        let text = date.map_or(String::new(), |date| {
            date.format(show_date_format).to_string()
        });
        show_date_text.set(text);
    });

    let on_input_blur = move |_| {
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
        validate.run(Some(DatePickerRuleTrigger::Blur));
    };

    let close_panel = move |date: Option<NaiveDate>| {
        if value.get_untracked() != date {
            if date.is_some() {
                value.set(date);
            }
            update_show_date_text();
        }
        is_show_panel.set(false);
    };

    let open_panel = move || {
        if is_show_panel.get() {
            return;
        }
        panel_selected_date.set(value.get_untracked());
        if let Some(panel_ref) = panel_ref.get_untracked() {
            panel_ref.init_panel(value.get_untracked().unwrap_or(now_date()));
        }
        is_show_panel.set(true);
    };

    view! {
        <Binder target_ref=date_picker_ref>
            <div node_ref=date_picker_ref class=class_list!["thaw-date-picker", class] on:click=move |_| open_panel()>
                <Input id name value=show_date_text on_focus=move |_| open_panel() on_blur=on_input_blur>
                    <InputSuffix slot>
                        <Icon icon=icondata_ai::AiCalendarOutlined style="font-size: 18px" />
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

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum DatePickerRuleTrigger {
    #[default]
    Blur,
}

pub struct DatePickerRule(Rule<Option<NaiveDate>, DatePickerRuleTrigger>);

impl DatePickerRule {
    pub fn required(required: MaybeSignal<bool>) -> Self {
        Self::validator(move |value, name| {
            if required.get_untracked() && value.is_none() {
                let message = name.get_untracked().map_or_else(
                    || String::from("Please select!"),
                    |name| format!("Please select {name}!"),
                );
                Err(FieldValidationState::Error(message))
            } else {
                Ok(())
            }
        })
    }

    pub fn required_with_message(
        required: MaybeSignal<bool>,
        message: MaybeSignal<String>,
    ) -> Self {
        Self::validator(move |value, _| {
            if required.get_untracked() && value.is_none() {
                Err(FieldValidationState::Error(message.get_untracked()))
            } else {
                Ok(())
            }
        })
    }

    pub fn validator(
        f: impl Fn(&Option<NaiveDate>, Signal<Option<String>>) -> Result<(), FieldValidationState>
            + Send
            + Sync
            + 'static,
    ) -> Self {
        Self(Rule::validator(f))
    }

    pub fn with_trigger(self, trigger: DatePickerRuleTrigger) -> Self {
        Self(Rule::with_trigger(self.0, trigger))
    }
}

impl Deref for DatePickerRule {
    type Target = Rule<Option<NaiveDate>, DatePickerRuleTrigger>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
