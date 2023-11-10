use crate::{
    mount_style,
    teleport::Teleport,
    utils::{add_event_listener, EventListenerHandle},
};
use leptos::{
    html::{AnyElement, ElementDescriptor, ToHtmlElement},
    leptos_dom::helpers::WindowListenerHandle,
    *,
};

#[slot]
pub struct Follower {
    #[prop(into)]
    show: MaybeSignal<bool>,
    children: Children,
}

#[component]
pub fn Binder<El: ElementDescriptor + Clone + 'static>(
    #[prop(into)] target_ref: NodeRef<El>,
    follower: Follower,
    children: Children,
) -> impl IntoView {
    mount_style("binder", include_str!("./binder.css"));
    let Follower {
        show: follower_show,
        children: follower_children,
    } = follower;

    let scroll_listener = store_value(None::<Callback<()>>);
    let scrollable_element_handle_vec = store_value::<Vec<EventListenerHandle>>(vec![]);
    let resize_handle = store_value(None::<WindowListenerHandle>);

    let ensure_scroll_listener = move || {
        let mut cursor = target_ref.get_untracked().map(|target| target.into_any());
        let mut scrollable_element_vec = vec![];
        loop {
            cursor = get_scroll_parent(cursor);
            if let Some(cursor) = cursor.take() {
                scrollable_element_vec.push(cursor);
            } else {
                break;
            }
        }
        let handle_vec = scrollable_element_vec
            .into_iter()
            .map(|ele| {
                add_event_listener(ele, ev::scroll, move |_| {
                    if let Some(scroll_listener) = scroll_listener.get_value() {
                        scroll_listener.call(());
                    }
                })
            })
            .collect();
        scrollable_element_handle_vec.set_value(handle_vec);
    };

    let add_scroll_listener = move |listener: Callback<()>| {
        scroll_listener.update_value(|scroll_listener| {
            if scroll_listener.is_none() {
                ensure_scroll_listener();
            }
            *scroll_listener = Some(listener);
        })
    };

    let remove_scroll_listener = move |_| {
        scrollable_element_handle_vec.update_value(|vec| {
            vec.drain(..).into_iter().for_each(|handle| handle.remove());
        });
        scroll_listener.set_value(None);
    };

    let add_resize_listener = move |listener: Callback<()>| {
        resize_handle.update_value(move |resize_handle| {
            if let Some(handle) = resize_handle.take() {
                handle.remove();
            }
            let handle = window_event_listener(ev::resize, move |_| {
                listener.call(());
            });

            *resize_handle = Some(handle);
        });
    };

    let remove_resize_listener = move |_| {
        resize_handle.update_value(move |handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    };

    on_cleanup(move || {
        remove_scroll_listener(());
        remove_resize_listener(());
    });
    view! {
        {children()}
        <FollowerContainer
            show=follower_show
            target_ref
            add_scroll_listener
            remove_scroll_listener
            add_resize_listener
            remove_resize_listener
        >
            {follower_children()}
        </FollowerContainer>
    }
}

#[component]
fn FollowerContainer<El: ElementDescriptor + Clone + 'static>(
    show: MaybeSignal<bool>,
    target_ref: NodeRef<El>,
    #[prop(into)] add_scroll_listener: Callback<Callback<()>>,
    #[prop(into)] remove_scroll_listener: Callback<()>,
    #[prop(into)] add_resize_listener: Callback<Callback<()>>,
    #[prop(into)] remove_resize_listener: Callback<()>,
    children: Children,
) -> impl IntoView {
    let content_ref = create_node_ref::<html::Div>();
    let content_style = create_rw_signal(String::new());
    let sync_position: Callback<()> = Callback::new(move |_| {
        let Some(target_ref) = target_ref.get().map(|target| target.into_any()) else {
            return;
        };
        let target_rect = target_ref.get_bounding_client_rect();

        let mut style = String::new();
        style.push_str(&format!("width: {}px;", target_rect.width()));
        style.push_str(&format!(
            "transform: translateX({}px) translateY({}px);",
            target_rect.x(),
            target_rect.y() + target_rect.height()
        ));
        content_style.set(style);
    });

    let is_show = create_memo(move |_| {
        if target_ref.get().is_none() {
            return false;
        }
        if content_ref.get().is_none() {
            return false;
        }
        let is_show = show.get();
        if is_show {
            sync_position.call(());
            add_scroll_listener.call(sync_position);
            add_resize_listener.call(sync_position);
        } else {
            remove_scroll_listener.call(());
            remove_resize_listener.call(());
        }
        is_show
    });
    view! {
        <Teleport>
            <div class="thaw-binder-follower-container" style=move || {
                if is_show.get() {
                    ""
                } else {
                    "display: none;"
                }
            }>
                <div class="thaw-binder-follower-content" ref=content_ref style=move || content_style.get()>
                    {children()}
                </div>
            </div>
        </Teleport>
    }
}

fn get_scroll_parent(element: Option<HtmlElement<AnyElement>>) -> Option<HtmlElement<AnyElement>> {
    let Some(element) = element else {
        return None;
    };

    fn get_parent_element(element: HtmlElement<AnyElement>) -> Option<HtmlElement<AnyElement>> {
        if element.node_type() == 9 {
            None
        } else {
            element.parent_element().map(|ele| ele.to_leptos_element())
        }
    }
    let Some(parent_element) = get_parent_element(element) else {
        return None;
    };

    if parent_element.node_type() == 9 {
        return Some(parent_element);
    }

    if parent_element.node_type() == 1 {
        fn get_overflow(
            parent_element: &HtmlElement<AnyElement>,
        ) -> Option<(String, String, String)> {
            let Ok(Some(css_style_declaration)) = window().get_computed_style(parent_element)
            else {
                return None;
            };
            let Ok(overflow) = css_style_declaration.get_property_value("overflow") else {
                return None;
            };
            let Ok(overflow_x) = css_style_declaration.get_property_value("overflowX") else {
                return None;
            };
            let Ok(overflow_y) = css_style_declaration.get_property_value("overflowY") else {
                return None;
            };
            Some((overflow, overflow_x, overflow_y))
        }
        if let Some((overflow, overflow_x, overflow_y)) = get_overflow(&parent_element) {
            let overflow = format!("{overflow}{overflow_x}{overflow_y}");
            if overflow.contains("auto") {
                return Some(parent_element);
            }
            if overflow.contains("scroll") {
                return Some(parent_element);
            }
            if overflow.contains("overlay") {
                return Some(parent_element);
            }
        }
    }

    get_scroll_parent(Some(parent_element))
}
