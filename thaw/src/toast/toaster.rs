use super::{ToastOptions, ToastPosition, ToasterReceiver};
use crate::ConfigInjection;
use leptos::{either::Either, html, prelude::*, tachys::view::any_view::AnyView};
use send_wrapper::SendWrapper;
use std::{collections::HashMap, time::Duration};
use thaw_components::{CSSTransition, Teleport};
use thaw_utils::mount_style;

#[component]
pub fn Toaster(
    receiver: ToasterReceiver,
    #[prop(optional)] position: ToastPosition,
    #[prop(default = Duration::from_secs(3000))] timeout: Duration,
) -> impl IntoView {
    mount_style("toaster", include_str!("./toaster.css"));
    let config_provider = ConfigInjection::expect_context();
    let top_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let top_start_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let top_end_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_start_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let bottom_end_id_list = RwSignal::<Vec<uuid::Uuid>>::new(Default::default());
    let toasts = StoredValue::<HashMap<uuid::Uuid, (SendWrapper<AnyView<Dom>>, ToastOptions)>>::new(
        Default::default(),
    );

    let id_list = move |position: &ToastPosition| match position {
        ToastPosition::Top => top_id_list,
        ToastPosition::TopStart => top_start_id_list,
        ToastPosition::TopEnd => top_end_id_list,
        ToastPosition::Bottom => bottom_id_list,
        ToastPosition::BottomStart => bottom_start_id_list,
        ToastPosition::BottomEnd => bottom_end_id_list,
    };

    Effect::new(move |_| {
        for (view, mut options) in receiver.try_recv() {
            if options.position.is_none() {
                options.position = Some(position);
            }
            if options.timeout.is_none() {
                options.timeout = Some(timeout);
            }

            let list = id_list(&options.position.unwrap());
            let id = options.id;
            toasts.update_value(|map| {
                map.insert(id, (SendWrapper::new(view), options));
            });
            list.update(|list| {
                list.push(id);
            });
        }
    });

    let on_close = move |(id, position)| {
        let list = id_list(&position);
        list.update(move |list| {
            let Some(index) = list.iter().position(|item_id| &id == item_id) else {
                return;
            };
            list.remove(index);
        });
    };

    view! {
        <Teleport>
            <div
                class="thaw-config-provider thaw-toaster-wrapper"
                data-thaw-id=config_provider.id().clone()
            >
                <div class="thaw-toaster thaw-toaster--top">
                    <For
                        each=move || top_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--top-start">
                    <For
                        each=move || top_start_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--top-end">
                    <For
                        each=move || top_end_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom">
                    <For
                        each=move || bottom_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom-start">
                    <For
                        each=move || bottom_start_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
                <div class="thaw-toaster thaw-toaster--bottom-end">
                    <For
                        each=move || bottom_end_id_list.get()
                        key=|id| id.clone()
                        let:id
                    >
                        {
                            if let Some((view, options)) = toasts.try_update_value(|map| { map.remove(&id) }).flatten() {
                                Either::Left(view! { <ToasterContainer on_close view=view.take() options/> })
                            } else {
                                Either::Right(())
                            }
                        }
                    </For>
                </div>
            </div>
        </Teleport>
    }
}

#[component]
fn ToasterContainer(
    view: AnyView<Dom>,
    options: ToastOptions,
    #[prop(into)] on_close: Callback<(uuid::Uuid, ToastPosition)>,
) -> impl IntoView {
    let container_ref = NodeRef::<html::Div>::new();
    let is_show = RwSignal::new(true);
    let ToastOptions {
        id,
        timeout,
        position,
        ..
    } = options;
    let timeout = timeout.unwrap();
    let position = position.unwrap();

    if !timeout.is_zero() {
        set_timeout(
            move || {
                is_show.set(false);
            },
            timeout,
        );
    }

    let on_before_leave = move |_| {
        let Some(el) = container_ref.get_untracked() else {
            return;
        };
        el.style(("max-height", format!("{}px", el.offset_height())));
    };
    let on_after_leave = move |_| {
        request_animation_frame(move || on_close.call((id, position)));
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
            <div class="thaw-toaster-container" node_ref=container_ref>
                {view}
            </div>
        </CSSTransition>
    }
}
