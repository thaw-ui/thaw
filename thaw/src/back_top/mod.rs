use crate::{ConfigInjection, Icon};
use leptos::{either::Either, ev, html, prelude::*};
use thaw_components::{CSSTransition, Teleport};
use thaw_utils::{
    add_event_listener, class_list, get_scroll_parent, mount_style, BoxCallback,
    EventListenerHandle,
};

#[component]
pub fn BackTop(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// The width of BackTop from the right side of the page.
    #[prop(default=40.into(), into)]
    right: MaybeSignal<i32>,
    /// The height of BackTop from the bottom of the page.
    #[prop(default=40.into(), into)]
    bottom: MaybeSignal<i32>,
    /// BackTop's trigger scroll top.
    #[prop(default=180.into(), into)]
    visibility_height: MaybeSignal<i32>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("back-top", include_str!("./back-top.css"));
    let config_provider = ConfigInjection::expect_context();
    let placeholder_ref = NodeRef::<html::Div>::new();
    let back_top_ref = NodeRef::<html::Div>::new();
    let is_show_back_top = RwSignal::new(false);
    let scroll_top = RwSignal::new(0);

    Effect::new(move |prev: Option<()>| {
        scroll_top.track();
        if prev.is_some() {
            is_show_back_top.set(scroll_top.get() > visibility_height.get_untracked());
        }
    });

    let scroll_to_top = StoredValue::new(None::<BoxCallback>);
    let scroll_handle = StoredValue::new(None::<EventListenerHandle>);

    Effect::new(move |_| {
        let Some(placeholder_el) = placeholder_ref.get() else {
            return;
        };

        request_animation_frame(move || {
            let scroll_el = get_scroll_parent(&placeholder_el)
                .unwrap_or_else(|| document().document_element().unwrap());

            {
                let scroll_el = send_wrapper::SendWrapper::new(scroll_el.clone());
                scroll_to_top.set_value(Some(BoxCallback::new(move || {
                    let options = web_sys::ScrollToOptions::new();
                    options.set_top(0.0);
                    options.set_behavior(web_sys::ScrollBehavior::Smooth);
                    scroll_el.scroll_to_with_scroll_to_options(&options);
                })));
            }

            let handle = add_event_listener(scroll_el.clone(), ev::scroll, move |_| {
                scroll_top.set(scroll_el.scroll_top());
            });
            scroll_handle.set_value(Some(handle));
        });
    });

    on_cleanup(move || {
        scroll_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    });

    let on_click = move |_| {
        scroll_to_top.with_value(|scroll_to_top| {
            if let Some(scroll_to_top) = scroll_to_top {
                scroll_to_top();
            }
        });
    };

    view! {
        <div style="display: none" class="thaw-back-top-placeholder" node_ref=placeholder_ref>
            <Teleport immediate=is_show_back_top>
                <CSSTransition
                    node_ref=back_top_ref
                    name="fade-in-scale-up-transition"
                    appear=is_show_back_top.get_untracked()
                    show=is_show_back_top
                    let:display
                >
                    <div
                        class=class_list!["thaw-config-provider thaw-back-top", class]
                        data-thaw-id=config_provider.id()
                        node_ref=back_top_ref
                        style=move || {
                            display
                                .get()
                                .map_or_else(
                                    || {
                                        format!(
                                            "right: {}px; bottom: {}px",
                                            right.get(),
                                            bottom.get(),
                                        )
                                    },
                                    |d| d.to_string(),
                                )
                        }
                        on:click=on_click
                    >
                        {if let Some(children) = children {
                            Either::Left(children())
                        } else {
                            Either::Right(
                                view! { <Icon icon=icondata_ai::AiVerticalAlignTopOutlined /> },
                            )
                        }}
                    </div>
                </CSSTransition>
            </Teleport>
        </div>
    }
}
