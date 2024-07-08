use leptos::{ev, html::ElementType, prelude::*};
use std::{ops::Deref, time::Duration};
use thaw_utils::{add_event_listener, EventListenerHandle, NextFrame};
use web_sys::wasm_bindgen::JsCast;

/// # CSS Transition
///
/// Reference to https://vuejs.org/guide/built-ins/transition.html
#[component]
pub fn CSSTransition<E, CF, IV>(
    node_ref: NodeRef<E>,
    #[prop(into)] show: MaybeSignal<bool>,
    #[prop(into)] name: MaybeSignal<String>,
    #[prop(optional)] appear: bool,
    #[prop(optional, into)] on_before_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_after_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_before_leave: Option<Callback<()>>,
    #[prop(optional, into)] on_leave: Option<Callback<()>>,
    #[prop(optional, into)] on_after_leave: Option<Callback<()>>,
    children: CF,
) -> impl IntoView
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + Deref<Target = web_sys::HtmlElement> + 'static,
    CF: FnOnce(ReadSignal<Option<&'static str>>) -> IV + Send + 'static,
    IV: IntoView + 'static,
{
    let display = RwSignal::new((!show.get_untracked()).then_some("display: none;"));
    let next_frame = NextFrame::new();
    let end_handle = StoredValue::new(None::<EventListenerHandle>);
    let end_count = StoredValue::new(None::<usize>);
    let finish = StoredValue::new(None::<Box<dyn FnOnce() + Send + Sync>>);

    Effect::new(move |_| {
        let target_ref = node_ref.get();
        let Some(el) = target_ref.as_deref() else {
            return;
        };

        let class_list = el.class_list();

        let on_end = {
            let el = send_wrapper::SendWrapper::new(el.clone());
            move |remove: Box<dyn FnOnce() + Send + Sync>| {
                let Some(CSSTransitionInfo {
                    types,
                    prop_count,
                    timeout,
                }) = get_transition_info(&el)
                else {
                    remove();
                    return;
                };

                finish.set_value(Some(Box::new(move || {
                    end_count.set_value(None);
                    remove();
                    end_handle.update_value(|h| {
                        h.take().map(|h| {
                            h.remove();
                        });
                    });
                })));

                set_timeout(
                    move || {
                        if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                            f();
                        }
                    },
                    Duration::from_millis(timeout + 1),
                );

                end_count.set_value(Some(0));
                let event_listener = move || {
                    end_count.update_value(|v| {
                        let Some(v) = v else {
                            return;
                        };
                        *v += 1;
                    });
                    if end_count.with_value(|v| {
                        let Some(v) = v else {
                            return false;
                        };
                        *v >= prop_count
                    }) {
                        if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                            f();
                        }
                    }
                };
                let handle = match types {
                    AnimationTypes::Transition => add_event_listener(
                        el.deref().clone().into(),
                        ev::transitionend,
                        move |_| event_listener(),
                    ),
                    AnimationTypes::Animation => {
                        add_event_listener(el.deref().clone().into(), ev::animationend, move |_| {
                            event_listener()
                        })
                    }
                };
                end_handle.set_value(Some(handle));
            }
        };

        let on_finish = move || {
            if let Some(Some(f)) = finish.try_update_value(|f| f.take()) {
                f();
            }
        };

        let name = name.clone();
        let effect = RenderEffect::new(move |prev: Option<bool>| {
            let show = show.get();
            let prev = if let Some(prev) = prev {
                prev
            } else if show && appear {
                false
            } else {
                return show;
            };

            let name = name.get_untracked();

            if show && !prev {
                on_finish();
                {
                    // on_enter
                    if let Some(on_before_enter) = on_before_enter.as_ref() {
                        on_before_enter.call(());
                    }
                    let enter_from = format!("{name}-enter-from");
                    let enter_active = format!("{name}-enter-active");
                    let enter_to = format!("{name}-enter-to");

                    let _ = class_list.add_2(&enter_from, &enter_active);
                    display.set(None);

                    let class_list = class_list.clone();
                    let on_end = on_end.clone();
                    next_frame.run(move || {
                        let _ = class_list.remove_1(&enter_from);
                        let _ = class_list.add_1(&enter_to);

                        let class_list = send_wrapper::SendWrapper::new(class_list);
                        let remove = Box::new(move || {
                            let _ = class_list.remove_2(&enter_active, &enter_to);
                            if let Some(on_after_enter) = on_after_enter.as_ref() {
                                on_after_enter.call(());
                            }
                        });
                        on_end(remove);

                        if let Some(on_enter) = on_enter.as_ref() {
                            on_enter.call(());
                        }
                    });
                }
            } else if !show && prev {
                on_finish();
                {
                    // on_leave
                    if let Some(on_before_leave) = on_before_leave.as_ref() {
                        on_before_leave.call(());
                    }
                    let leave_from = format!("{name}-leave-from");
                    let leave_active = format!("{name}-leave-active");
                    let leave_to = format!("{name}-leave-to");

                    let _ = class_list.add_2(&leave_from, &leave_active);

                    let class_list = class_list.clone();
                    let on_after_leave = on_after_leave.clone();
                    let on_end = on_end.clone();
                    let on_leave = on_leave.clone();
                    next_frame.run(move || {
                        let _ = class_list.remove_1(&leave_from);
                        let _ = class_list.add_1(&leave_to);

                        let class_list = send_wrapper::SendWrapper::new(class_list);
                        let remove = Box::new(move || {
                            let _ = class_list.remove_2(&leave_active, &leave_to);
                            display.set(Some("display: none;"));
                            if let Some(on_after_leave) = on_after_leave.as_ref() {
                                on_after_leave.call(());
                            }
                        });
                        on_end(remove);
                        if let Some(on_leave) = on_leave {
                            on_leave.call(());
                        }
                    });
                }
            }

            show
        });

        on_cleanup(move || {
            drop(effect);
            end_handle.update_value(|handle| {
                if let Some(handle) = handle.take() {
                    handle.remove();
                }
            });
        })
    });

    children(display.read_only())
}

#[derive(PartialEq)]
enum AnimationTypes {
    Transition,
    Animation,
}

struct CSSTransitionInfo {
    types: AnimationTypes,
    prop_count: usize,
    timeout: u64,
}

fn get_transition_info(el: &web_sys::Element) -> Option<CSSTransitionInfo> {
    let styles = window().get_computed_style(el).ok().flatten()?;

    let get_style_properties = |property: &str| {
        styles
            .get_property_value(property)
            .unwrap_or_default()
            .split(", ")
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
    };

    let transition_delays = get_style_properties("transition-delay");
    let transition_durations = get_style_properties("transition-duration");
    let transition_timeout = get_timeout(transition_delays, &transition_durations);
    let animation_delays = get_style_properties("animation-delay");
    let animation_durations = get_style_properties("animation-duration");
    let animation_timeout = get_timeout(animation_delays, &animation_durations);

    let timeout = u64::max(transition_timeout, animation_timeout);
    let (types, prop_count) = if timeout > 0 {
        if transition_timeout > animation_timeout {
            (AnimationTypes::Transition, transition_durations.len())
        } else {
            (AnimationTypes::Animation, animation_durations.len())
        }
    } else {
        return None;
    };

    Some(CSSTransitionInfo {
        types,
        prop_count,
        timeout,
    })
}

fn get_timeout(mut delays: Vec<String>, durations: &Vec<String>) -> u64 {
    while delays.len() < durations.len() {
        delays.append(&mut delays.clone())
    }

    fn to_ms(s: &String) -> u64 {
        if s == "auto" || s.is_empty() {
            return 0;
        }

        let s = if s.ends_with("ms") {
            s.split_at(s.len() - 2).0
        } else {
            s.split_at(s.len() - 1).0
        };

        (s.parse::<f32>().unwrap_or_default() * 1000.0).floor() as u64
    }

    durations
        .iter()
        .enumerate()
        .map(|(i, d)| to_ms(d) + to_ms(&delays[i]))
        .max()
        .unwrap_or_default()
}
