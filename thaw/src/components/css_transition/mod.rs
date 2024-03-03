use leptos::{html::ElementDescriptor, *};
use std::ops::Deref;

use crate::utils::{add_event_listener, EventListenerHandle};

/// # CSS Transition
///
/// Reference to https://vuejs.org/guide/built-ins/transition.html
#[component]
pub fn CSSTransition<T, CF, IV>(
    node_ref: NodeRef<T>,
    #[prop(into)] show: MaybeSignal<bool>,
    #[prop(into)] name: MaybeSignal<String>,
    #[prop(optional, into)] on_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_after_enter: Option<Callback<()>>,
    #[prop(optional, into)] on_after_leave: Option<Callback<()>>,
    children: CF,
) -> impl IntoView
where
    T: ElementDescriptor + Clone + 'static,
    CF: FnOnce(ReadSignal<Option<&'static str>>) -> IV + 'static,
    IV: IntoView,
{
    let display = create_rw_signal((!show.get_untracked()).then_some("display: none;"));

    node_ref.on_load(move |node_el| {
        let any_el = node_el.clone().into_any();
        let el = any_el.deref();
        let class_list = el.class_list();
        let handle = StoredValue::new(None::<(EventListenerHandle, EventListenerHandle)>);

        let on_end = Callback::new(move |remove: Callback<()>| {
            let transition_handle =
                add_event_listener(any_el.clone(), ev::transitionend, move |_| {
                    remove.call(());
                    handle.update_value(|h| {
                        h.take().map(|(t, a)| {
                            t.remove();
                            a.remove();
                        });
                    });
                });
            let animation_handle =
                add_event_listener(any_el.clone(), ev::animationend, move |_| {
                    remove.call(());
                    handle.update_value(|h| {
                        h.take().map(|(t, a)| {
                            t.remove();
                            a.remove();
                        });
                    });
                });
            handle.set_value(Some((transition_handle, animation_handle)));
        });

        let on_enter_fn = {
            let class_list = class_list.clone();
            Callback::new(move |name: String| {
                let enter_from = format!("{name}-enter-from");
                let enter_active = format!("{name}-enter-active");
                let enter_to = format!("{name}-enter-to");

                let _ = class_list.add_2(&enter_from, &enter_active);
                display.set(None);

                let class_list = class_list.clone();
                next_frame(move || {
                    let _ = class_list.remove_1(&enter_from);
                    let _ = class_list.add_1(&enter_to);

                    let remove = Callback::new(move |_| {
                        let _ = class_list.remove_2(&enter_active, &enter_to);
                        if let Some(on_after_enter) = on_after_enter {
                            on_after_enter.call(());
                        }
                    });
                    on_end.call(remove);

                    if let Some(on_enter) = on_enter {
                        on_enter.call(());
                    }
                });
            })
        };

        let on_leave_fn = {
            let class_list = class_list.clone();
            Callback::new(move |name: String| {
                let leave_from = format!("{name}-leave-from");
                let leave_active = format!("{name}-leave-active");
                let leave_to = format!("{name}-leave-to");

                let _ = class_list.add_2(&leave_from, &leave_active);

                let class_list = class_list.clone();
                next_frame(move || {
                    let _ = class_list.remove_1(&leave_from);
                    let _ = class_list.add_1(&leave_to);

                    let remove = Callback::new(move |_| {
                        let _ = class_list.remove_2(&leave_active, &leave_to);
                        display.set(Some("display: none;"));
                        if let Some(on_after_leave) = on_after_leave {
                            on_after_leave.call(());
                        }
                    });
                    on_end.call(remove);
                });
            })
        };

        create_render_effect(move |prev: Option<bool>| {
            let show = show.get();
            let Some(prev) = prev else {
                return show;
            };

            let name = name.get_untracked();

            if show && !prev {
                on_enter_fn.call(name);
            } else if !show && prev {
                on_leave_fn.call(name);
            }

            show
        });
    });

    children(display.read_only())
}


pub fn next_frame(cb: impl FnOnce() + 'static) {
    request_animation_frame(move || {
        request_animation_frame(cb);
    });
}