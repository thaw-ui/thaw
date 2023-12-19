mod theme;

use crate::{
    chrono::{Local, NaiveTime, Timelike},
    components::{Binder, Follower, FollowerPlacement},
    use_theme,
    utils::{mount_style, ComponentRef},
    AiIcon, Button, ButtonSize, ButtonVariant, Icon, Input, InputSuffix, SignalWatch, Theme,
};
use leptos::*;
pub use theme::TimePickerTheme;

#[component]
pub fn TimePicker(
    #[prop(optional, into)] value: RwSignal<Option<NaiveTime>>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("time-picker", include_str!("./time-picker.css"));
    let time_picker_ref = create_node_ref::<html::Div>();
    let panel_ref = ComponentRef::<PanelRef>::default();
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
    let panel_selected_time = create_rw_signal(None::<NaiveTime>);
    _ = panel_selected_time.watch(move |time| {
        let text = time.as_ref().map_or(String::new(), |time| {
            time.format(show_time_format).to_string()
        });
        show_time_text.set(text);
    });

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
    let close_panel = Callback::new(move |time: Option<NaiveTime>| {
        if value.get_untracked() != time {
            if time.is_some() {
                value.set(time);
            }
            update_show_time_text();
        }
        is_show_panel.set(false);
    });

    let open_panel = Callback::new(move |_| {
        panel_selected_time.set(value.get_untracked());
        is_show_panel.set(true);
        request_animation_frame(move || {
            if let Some(panel_ref) = panel_ref.get_untracked() {
                panel_ref.scroll_into_view();
            }
        });
    });

    view! {
        <Binder target_ref=time_picker_ref>
            <div ref=time_picker_ref>
                <Input
                    class
                    value=show_time_text
                    on_focus=open_panel
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
            <Follower slot show=is_show_panel placement=FollowerPlacement::BottomStart>
                <Panel
                    selected_time=panel_selected_time
                    close_panel
                    time_picker_ref
                    comp_ref=panel_ref
                />
            </Follower>
        </Binder>
    }
}

#[component]
fn Panel(
    selected_time: RwSignal<Option<NaiveTime>>,
    time_picker_ref: NodeRef<html::Div>,
    close_panel: Callback<Option<NaiveTime>>,
    comp_ref: ComponentRef<PanelRef>,
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
                theme.time_picker.panel_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-item-background-color-hover: {};",
                theme.time_picker.panel_time_item_background_color_hover
            ));
            css_vars.push_str(&format!(
                "--thaw-item-border-color: {};",
                theme.time_picker.panel_border_color
            ));
        });
        css_vars
    });
    let now = Callback::new(move |_| {
        close_panel.call(Some(now_time()));
    });
    let ok = Callback::new(move |_| {
        close_panel.call(selected_time.get_untracked());
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
                    || current_el == ***time_picker_ref.get_untracked().unwrap()
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
        _ = time_picker_ref;
        _ = panel_ref;
    }

    let hour_ref = create_node_ref::<html::Div>();
    let minute_ref = create_node_ref::<html::Div>();
    let second_ref = create_node_ref::<html::Div>();
    comp_ref.load(PanelRef {
        hour_ref,
        minute_ref,
        second_ref,
    });

    view! {
        <div class="thaw-time-picker-panel" style=move || css_vars.get() ref=panel_ref>
            <div class="thaw-time-picker-panel__time">
                <div class="thaw-time-picker-panel__time-hour" ref=hour_ref>

                    {(0..24)
                        .map(|hour| {
                            let comp_ref = ComponentRef::<PanelTimeItemRef>::default();
                            let on_click = move |_| {
                                selected_time
                                    .update(move |time| {
                                        *time = if let Some(time) = time {
                                            time.with_hour(hour)
                                        } else {
                                            NaiveTime::from_hms_opt(hour, 0, 0)
                                        }
                                    });
                                comp_ref.get_untracked().unwrap().scroll_into_view();
                            };
                            let is_selected = Memo::new(move |_| {
                                selected_time.get().map_or(false, |v| v.hour() == hour)
                            });
                            view! {
                                <PanelTimeItem value=hour on:click=on_click is_selected comp_ref/>
                            }
                        })
                        .collect_view()}
                    <div class="thaw-time-picker-panel__time-padding"></div>
                </div>
                <div class="thaw-time-picker-panel__time-minute" ref=minute_ref>

                    {(0..60)
                        .map(|minute| {
                            let comp_ref = ComponentRef::<PanelTimeItemRef>::default();
                            let on_click = move |_| {
                                selected_time
                                    .update(move |time| {
                                        *time = if let Some(time) = time {
                                            time.with_minute(minute)
                                        } else {
                                            NaiveTime::from_hms_opt(now_time().hour(), minute, 0)
                                        }
                                    });
                                comp_ref.get_untracked().unwrap().scroll_into_view();
                            };
                            let is_selected = Memo::new(move |_| {
                                selected_time.get().map_or(false, |v| v.minute() == minute)
                            });
                            view! {
                                <PanelTimeItem value=minute on:click=on_click is_selected comp_ref/>
                            }
                        })
                        .collect_view()}
                    <div class="thaw-time-picker-panel__time-padding"></div>
                </div>
                <div class="thaw-time-picker-panel__time-second" ref=second_ref>

                    {(0..60)
                        .map(|second| {
                            let comp_ref = ComponentRef::<PanelTimeItemRef>::default();
                            let on_click = move |_| {
                                selected_time
                                    .update(move |time| {
                                        *time = if let Some(time) = time {
                                            time.with_second(second)
                                        } else {
                                            now_time().with_second(second)
                                        }
                                    });
                                comp_ref.get_untracked().unwrap().scroll_into_view();
                            };
                            let is_selected = Memo::new(move |_| {
                                selected_time.get().map_or(false, |v| v.second() == second)
                            });
                            view! {
                                <PanelTimeItem value=second on:click=on_click is_selected comp_ref/>
                            }
                        })
                        .collect_view()}
                    <div class="thaw-time-picker-panel__time-padding"></div>
                </div>
            </div>
            <div class="thaw-time-picker-panel__footer">
                <Button variant=ButtonVariant::Solid size=ButtonSize::Tiny on_click=now>
                    "Now"
                </Button>
                <Button size=ButtonSize::Tiny on_click=ok>
                    "OK"
                </Button>
            </div>
        </div>
    }
}

#[derive(Clone)]
struct PanelRef {
    hour_ref: NodeRef<html::Div>,
    minute_ref: NodeRef<html::Div>,
    second_ref: NodeRef<html::Div>,
}

impl PanelRef {
    fn scroll_top(el: HtmlElement<html::Div>) {
        if let Ok(Some(slected_el)) =
            el.query_selector(".thaw-time-picker-panel__time-item--slected")
        {
            use wasm_bindgen::JsCast;
            if let Ok(slected_el) = slected_el.dyn_into::<web_sys::HtmlElement>() {
                el.set_scroll_top(slected_el.offset_top());
            }
        }
    }

    fn scroll_into_view(&self) {
        if let Some(hour_el) = self.hour_ref.get_untracked() {
            Self::scroll_top(hour_el);
        }
        if let Some(minute_el) = self.minute_ref.get_untracked() {
            Self::scroll_top(minute_el);
        }
        if let Some(second_el) = self.second_ref.get_untracked() {
            Self::scroll_top(second_el);
        }
    }
}

#[component]
fn PanelTimeItem(
    value: u32,
    is_selected: Memo<bool>,
    comp_ref: ComponentRef<PanelTimeItemRef>,
) -> impl IntoView {
    let item_ref = create_node_ref();
    item_ref.on_load(move |_| {
        let item_ref = PanelTimeItemRef { item_ref };
        comp_ref.load(item_ref);
    });
    view! {
        <div
            class="thaw-time-picker-panel__time-item"
            class=("thaw-time-picker-panel__time-item--slected", move || is_selected.get())
            ref=item_ref
        >

            {format!("{value:02}")}

        </div>
    }
}

#[derive(Clone)]
struct PanelTimeItemRef {
    item_ref: NodeRef<html::Div>,
}

impl PanelTimeItemRef {
    fn scroll_into_view(&self) {
        if let Some(item_ref) = self.item_ref.get_untracked() {
            item_ref.scroll_into_view_with_bool(true);
        }
    }
}

fn now_time() -> NaiveTime {
    Local::now().time()
}

