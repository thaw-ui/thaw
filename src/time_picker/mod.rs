mod theme;

use crate::{
    chrono::NaiveTime,
    components::{Binder, Follower, FollowerPlacement, FollowerWidth},
    use_theme,
    utils::mount_style,
    AiIcon, Icon, Input, InputSuffix, Theme,
};
use leptos::*;
pub use theme::TimePickerTheme;

#[component]
pub fn TimePicker(#[prop(optional, into)] value: RwSignal<Option<NaiveTime>>) -> impl IntoView {
    mount_style("time-picker", include_str!("./time-picker.css"));
    let time_picker_ref = create_node_ref::<html::Div>();
    let is_show_panel = create_rw_signal(false);
    let show_time_format = "%H:%M:%S";
    let show_time_text = create_rw_signal(String::new());
    let update_show_time_text = move || {
        value.with_untracked(move |time| {
            let text = time.as_ref().map_or(String::new(), |time| {
                time.format(show_time_format).to_string()
            });
            show_time_text.set(text);
        });
    };
    update_show_time_text();

    let on_input_blur = Callback::new(move |_| {
        is_show_panel.set(false);
        if let Ok(time) =
            NaiveTime::parse_from_str(&show_time_text.get_untracked(), show_time_format)
        {
            if value.get_untracked() != Some(time) {
                value.set(Some(time));
                update_show_time_text();
            }
        } else {
            update_show_time_text();
        }
    });

    view! {
        <Binder target_ref=time_picker_ref>
            <div ref=time_picker_ref>
                <Input
                    value=show_time_text
                    on_focus=move |_| is_show_panel.set(true)
                    on_blur=on_input_blur
                >
                    <InputSuffix slot>
                        <Icon
                            icon=Icon::from(AiIcon::AiClockCircleOutlined)
                            style="font-size: 18px"
                        />
                    </InputSuffix>
                </Input>
            </div>
            <Follower
                slot
                show=is_show_panel
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <Panel/>
            </Follower>
        </Binder>
    }
}

#[component]
fn Panel() -> impl IntoView {
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.time_picker.panel_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-item-background-color-hover: {};",
                theme.time_picker.panel_time_item_background_color_hover
            ));
        });
        css_vars
    });

    view! {
        <div class="thaw-time-picker-panel" style=move || css_vars.get()>
            <div class="thaw-time-picker-panel__time">
                <div class="thaw-time-picker-panel__time-hour">

                    {(0..24)
                        .into_iter()
                        .map(|value| {
                            view! { <PanelTimeItem value/> }
                        })
                        .collect_view()}

                </div>
                <div class="thaw-time-picker-panel__time-minute">

                    {(0..60)
                        .into_iter()
                        .map(|value| {
                            view! { <PanelTimeItem value/> }
                        })
                        .collect_view()}

                </div>
                <div class="thaw-time-picker-panel__time-second">

                    {(0..60)
                        .into_iter()
                        .map(|value| {
                            view! { <PanelTimeItem value/> }
                        })
                        .collect_view()}

                </div>
            </div>
            <div class="thaw-time-picker-panel__footer">
            </div>
        </div>
    }
}

#[component]
fn PanelTimeItem(value: u8) -> impl IntoView {
    view! {
        <div class="thaw-time-picker-panel__time-item">

            {format!("{value:02}")}

        </div>
    }
}
