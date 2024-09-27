use super::{ToastIntent, ToastOptions, ToastPosition, ToasterReceiver};
use crate::{toast::ToasterMessage, ConfigInjection};
use leptos::{context::Provider, either::Either, html, prelude::*};
use send_wrapper::SendWrapper;
use std::{collections::HashMap, time::Duration};
use thaw_components::{CSSTransition, Teleport};
use thaw_utils::{mount_style, ArcTwoCallback};
use wasm_bindgen::UnwrapThrowExt;

#[component]
pub fn Toaster(
    receiver: ToasterReceiver,
    #[prop(optional)] position: ToastPosition,
    #[prop(optional)] intent: ToastIntent,
    #[prop(default = Duration::from_secs(3))] timeout: Duration,
) -> impl IntoView {
    mount_style("toaster", include_str!("./toaster.css"));
    let config_provider = ConfigInjection::expect_context();
    let top_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let top_start_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let top_end_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_start_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_end_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let toasts = StoredValue::<
        HashMap<uuid::Uuid, (SendWrapper<Children>, ToastOptions, RwSignal<bool>)>,
    >::new(Default::default());
    let toast_show_list =
        StoredValue::<HashMap<uuid::Uuid, RwSignal<bool>>>::new(Default::default());

    let id_list = move |position: &ToastPosition| match position {
        ToastPosition::Top => top_id_list,
        ToastPosition::TopStart => top_start_id_list,
        ToastPosition::TopEnd => top_end_id_list,
        ToastPosition::Bottom => bottom_id_list,
        ToastPosition::BottomStart => bottom_start_id_list,
        ToastPosition::BottomEnd => bottom_end_id_list,
    };

    let owner = Owner::current().unwrap();
    Effect::new(move |_| {
        for message in receiver.try_recv() {
            match message {
                ToasterMessage::Dispatch(view, mut options) => {
                    if options.position.is_none() {
                        options.position = Some(position);
                    }
                    if options.timeout.is_none() {
                        options.timeout = Some(timeout);
                    }
                    if options.intent.is_none() {
                        options.intent = Some(intent);
                    }

                    let list = id_list(&options.position.unwrap_throw());
                    let id = options.id;
                    let is_show = owner.with(|| RwSignal::new(true));
                    toasts.update_value(|map| {
                        map.insert(id, (SendWrapper::new(view), options, is_show));
                    });
                    toast_show_list.update_value(|map| {
                        map.insert(id, is_show);
                    });
                    list.update(|list| {
                        list.push(id);
                    });
                }
                ToasterMessage::Dismiss(toast_id) => {
                    toast_show_list.with_value(|map| {
                        map.get(&toast_id).unwrap_throw().set(false)
                    });
                }
            }
        }
    });

    let on_close = StoredValue::new(ArcTwoCallback::new(move |id, position| {
        let list = id_list(&position);
        list.update(move |list| {
            let Some(index) = list.iter().position(|item_id| &id == item_id) else {
                return;
            };
            list.remove(index);
        });
        toast_show_list.update_value(|map| map.remove(&id));
    }));

    view! {
        <Teleport>
            <div
                class="thaw-config-provider thaw-toaster-wrapper"
                data-thaw-id=config_provider.id()
            >
                <div class="thaw-toaster thaw-toaster--top">
                    <For each=move || top_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--top-start">
                    <For each=move || top_start_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--top-end">
                    <For each=move || top_end_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom">
                    <For each=move || bottom_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom-start">
                    <For each=move || bottom_start_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom-end">
                    <For each=move || bottom_end_id_list.get() key=|id| id.clone() let:id>
                        {if let Some((view, options, is_show)) = toasts
                            .try_update_value(|map| { map.remove(&id) })
                            .flatten()
                        {
                            Either::Left(
                                view! {
                                    <ToasterContainer
                                        on_close
                                        children=view.take()
                                        options
                                        is_show
                                    />
                                },
                            )
                        } else {
                            Either::Right(())
                        }}
                    </For>
                </div>
            </div>
        </Teleport>
    }
}

#[component]
fn ToasterContainer(
    options: ToastOptions,
    #[prop(into)] on_close: StoredValue<ArcTwoCallback<uuid::Uuid, ToastPosition>>,
    children: Children,
    is_show: RwSignal<bool>,
) -> impl IntoView {
    let container_ref = NodeRef::<html::Div>::new();
    let ToastOptions {
        id,
        timeout,
        position,
        intent,
        ..
    } = options;

    let timeout = timeout.unwrap_throw();
    let position = position.unwrap_throw();
    let intent = intent.unwrap_throw();

    if !timeout.is_zero() {
        set_timeout(
            move || {
                is_show.set(false);
            },
            timeout,
        );
    }

    let on_before_leave = move || {
        let Some(el) = container_ref.get_untracked() else {
            return;
        };
        el.style(("max-height", format!("{}px", el.offset_height())));
    };
    let on_after_leave = move || {
        request_animation_frame(move || {
            if let Some(f) = on_close.try_with_value(|f| f.clone()) {
                f(id, position);
            }
        });
    };

    view! {
        <CSSTransition
            node_ref=container_ref
            name="fade-in-height-expand-transition"
            show=is_show
            appear=true
            on_before_leave=on_before_leave
            on_after_leave=on_after_leave
            let:_
        >
            <Provider value=intent>
                <div class="thaw-toaster-container" node_ref=container_ref>
                    {children()}
                </div>
            </Provider>
        </CSSTransition>
    }
}
