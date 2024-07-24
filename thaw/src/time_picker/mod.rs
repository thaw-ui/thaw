use crate::{
    Button, ButtonSize, ConfigInjection, Icon, Input, InputSuffix, Scrollbar, ScrollbarRef,
    SignalWatch,
};
use chrono::{Local, NaiveTime, Timelike};
use leptos::{html, prelude::*};
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{mount_style, ComponentRef, OptionModel, OptionalProp};

#[component]
pub fn TimePicker(
    #[prop(optional, into)] value: OptionModel<NaiveTime>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    // #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
) -> impl IntoView {
    mount_style("time-picker", include_str!("./time-picker.css"));
    let time_picker_ref = NodeRef::<html::Div>::new();
    let panel_ref = ComponentRef::<PanelRef>::default();
    let is_show_panel = RwSignal::new(false);
    let show_time_format = "%H:%M:%S";
    let show_time_text = RwSignal::new(String::new());
    let update_show_time_text = move || {
        value.with_untracked(move |time| {
            let text = time.as_ref().map_or(String::new(), |time| {
                time.format(show_time_format).to_string()
            });
            show_time_text.set(text);
        });
    };
    update_show_time_text();
    let panel_selected_time = RwSignal::new(None::<NaiveTime>);
    _ = panel_selected_time.watch(move |time| {
        let text = time.as_ref().map_or(String::new(), |time| {
            time.format(show_time_format).to_string()
        });
        show_time_text.set(text);
    });

    let on_input_blur = move |_| {
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
    };
    let close_panel = Callback::new(move |time: Option<NaiveTime>| {
        if value.get_untracked() != time {
            if time.is_some() {
                value.set(time);
            }
            update_show_time_text();
        }
        is_show_panel.set(false);
    });

    let open_panel = move |_| {
        panel_selected_time.set(value.get_untracked());
        is_show_panel.set(true);
        request_animation_frame(move || {
            if let Some(panel_ref) = panel_ref.get_untracked() {
                panel_ref.scroll_into_view();
            }
        });
    };

    view! {
        <Binder target_ref=time_picker_ref>
            <div node_ref=time_picker_ref>
                <Input class value=show_time_text on_focus=open_panel on_blur=on_input_blur>
                    <InputSuffix slot>
                        <Icon icon=icondata_ai::AiClockCircleOutlined style="font-size: 18px"/>
                    </InputSuffix>
                </Input>
            </div>
            <Follower slot show=is_show_panel placement=FollowerPlacement::BottomStart>
                <Panel
                    selected_time=panel_selected_time
                    close_panel
                    time_picker_ref
                    is_show_panel
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
    #[prop(into)] is_show_panel: MaybeSignal<bool>,
    comp_ref: ComponentRef<PanelRef>,
) -> impl IntoView {
    let config_provider = ConfigInjection::use_();
    let now = move |_| {
        close_panel.call(Some(now_time()));
    };
    let ok = move |_| {
        close_panel.call(selected_time.get_untracked());
    };

    let panel_ref = NodeRef::<html::Div>::new();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let handle = window_event_listener(leptos::ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                let Some(panel_el) = panel_ref.get() else {
                    return;
                };
                let time_picker_el = time_picker_ref.get().unwrap();
                if current_el == **panel_el || current_el == **time_picker_el {
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

    let hour_ref = ComponentRef::<ScrollbarRef>::new();
    let minute_ref = ComponentRef::<ScrollbarRef>::new();
    let second_ref = ComponentRef::<ScrollbarRef>::new();
    comp_ref.load(PanelRef {
        hour_ref,
        minute_ref,
        second_ref,
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
                class="thaw-config-provider thaw-time-picker-panel"
                data-thaw-id=config_provider.id().clone()
                style=move || display.get().unwrap_or_default()
                node_ref=panel_ref
            >
                <div class="thaw-time-picker-panel__time">
                    <div class="thaw-time-picker-panel__time-hour">
                        <Scrollbar size=6 comp_ref=hour_ref>
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
                                        <PanelTimeItem
                                            value=hour
                                            on:click=on_click
                                            is_selected
                                            comp_ref
                                        />
                                    }
                                })
                                .collect_view()}
                            <div class="thaw-time-picker-panel__time-padding"></div>
                        </Scrollbar>
                    </div>
                    <div class="thaw-time-picker-panel__time-minute">
                        <Scrollbar size=6 comp_ref=minute_ref>
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
                                        <PanelTimeItem
                                            value=minute
                                            on:click=on_click
                                            is_selected
                                            comp_ref
                                        />
                                    }
                                })
                                .collect_view()}
                            <div class="thaw-time-picker-panel__time-padding"></div>
                        </Scrollbar>
                    </div>
                    <div class="thaw-time-picker-panel__time-second">
                        <Scrollbar size=6 comp_ref=second_ref>
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
                                        <PanelTimeItem
                                            value=second
                                            on:click=on_click
                                            is_selected
                                            comp_ref
                                        />
                                    }
                                })
                                .collect_view()}
                            <div class="thaw-time-picker-panel__time-padding"></div>
                        </Scrollbar>
                    </div>
                </div>
                <div class="thaw-time-picker-panel__footer">
                    <Button size=ButtonSize::Small on_click=now>
                        "Now"
                    </Button>
                    <Button size=ButtonSize::Small on_click=ok>
                        "OK"
                    </Button>
                </div>
            </div>
        </CSSTransition>
    }
}

#[derive(Clone)]
struct PanelRef {
    hour_ref: ComponentRef<ScrollbarRef>,
    minute_ref: ComponentRef<ScrollbarRef>,
    second_ref: ComponentRef<ScrollbarRef>,
}

impl PanelRef {
    fn scroll_top(scrollbar_ref: ScrollbarRef) {
        let Some(contetn_ref) = scrollbar_ref.content_ref.get_untracked() else {
            return;
        };
        let Ok(Some(slected_el)) =
            contetn_ref.query_selector(".thaw-time-picker-panel__time-item--slected")
        else {
            return;
        };
        use wasm_bindgen::JsCast;
        if let Ok(slected_el) = slected_el.dyn_into::<web_sys::HtmlElement>() {
            scrollbar_ref.scroll_to_with_scroll_to_options(
                web_sys::ScrollToOptions::new().top(f64::from(slected_el.offset_top())),
            );
        }
    }

    fn scroll_into_view(&self) {
        if let Some(hour) = self.hour_ref.get_untracked() {
            Self::scroll_top(hour);
        }
        if let Some(minute) = self.minute_ref.get_untracked() {
            Self::scroll_top(minute);
        }
        if let Some(second) = self.second_ref.get_untracked() {
            Self::scroll_top(second);
        }
    }
}

#[component]
fn PanelTimeItem(
    value: u32,
    is_selected: Memo<bool>,
    comp_ref: ComponentRef<PanelTimeItemRef>,
) -> impl IntoView {
    let item_ref = NodeRef::new();
    comp_ref.load(PanelTimeItemRef { item_ref });

    view! {
        <div
            class="thaw-time-picker-panel__time-item"
            class=("thaw-time-picker-panel__time-item--slected", move || is_selected.get())
            node_ref=item_ref
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
