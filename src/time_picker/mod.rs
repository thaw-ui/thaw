use crate::{
    chrono::NaiveTime,
    components::{Binder, Follower, FollowerPlacement},
    AiIcon, Icon, Input, InputSuffix,
};
use leptos::*;

#[component]
pub fn TimePicker(#[prop(optional, into)] value: RwSignal<Option<NaiveTime>>) -> impl IntoView {
    let time_picker_ref = create_node_ref::<html::Div>();
    let is_show_picker = create_rw_signal(false);
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
                <Input value=show_time_text on_blur=on_input_blur>
                    <InputSuffix slot>
                        <Icon icon=Icon::from(AiIcon::AiClockCircleOutlined) style="font-size: 18px"/>
                    </InputSuffix>
                </Input>
            </div>
            <Follower slot show=is_show_picker placement=FollowerPlacement::BottomStart >
                <div>

                </div>
            </Follower>
        </Binder>
    }
}
