use super::{
    get_placement_style::{get_follower_placement_offset, FollowerPlacementOffset},
    Follower, FollowerInjection, FollowerPlacement, FollowerWidth,
};
use leptos::{ev, html, leptos_dom::helpers::WindowListenerHandle, logging, prelude::*};
use thaw_utils::{add_event_listener, get_scroll_parent_node, mount_style, EventListenerHandle};
use web_sys::wasm_bindgen::UnwrapThrowExt;

pub struct UseBinder<FC> {
    pub target_ref: NodeRef<thaw_utils::Element>,
    pub content_ref: NodeRef<thaw_utils::HtmlElement>,
    pub follower_show: Signal<bool>,
    pub follower_ref: NodeRef<html::Div>,
    pub follower_injection: FollowerInjection,
    pub follower_children: TypedChildren<FC>,
    pub placement: RwSignal<FollowerPlacement>,
}

pub fn use_binder<FC>(follower: Follower<FC>) -> UseBinder<FC>
where
    FC: AddAnyAttr + IntoView + Send + 'static,
{
    mount_style("binder", include_str!("./binder.css"));
    let Follower {
        show: follower_show,
        width: follower_width,
        placement: follower_placement,
        children: follower_children,
    } = follower;

    let scrollable_element_handle_vec = StoredValue::<Vec<EventListenerHandle>>::new(vec![]);
    let resize_handle = StoredValue::new(None::<WindowListenerHandle>);
    let target_ref = NodeRef::<thaw_utils::Element>::new();
    let follower_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<thaw_utils::HtmlElement>::new();
    let placement = RwSignal::new(follower_placement);
    let sync_position = move || {
        let Some(follower_el) = follower_ref.get_untracked() else {
            return;
        };
        let Some(content_ref) = content_ref.get_untracked() else {
            return;
        };
        let Some(target_ref) = target_ref.get_untracked() else {
            return;
        };
        let follower_rect = follower_el.get_bounding_client_rect();
        let target_rect = target_ref.get_bounding_client_rect();
        let content_rect = content_ref.get_bounding_client_rect();
        let mut styles = Vec::<(&str, String)>::new();
        if let Some(width) = follower_width {
            match width {
                FollowerWidth::Target => {
                    styles.push(("width", format!("{}px", target_rect.width())))
                }
                FollowerWidth::MinTarget => {
                    styles.push(("min-width", format!("{}px", target_rect.width())))
                }
                FollowerWidth::Px(width) => styles.push(("width", format!("{width}px"))),
            };
        }
        if let Some(FollowerPlacementOffset {
            top,
            left,
            transform,
            placement: new_placement,
            max_height,
        }) = get_follower_placement_offset(
            follower_placement,
            target_rect,
            follower_rect,
            content_rect,
        ) {
            if let Some(max_height) = max_height {
                styles.push(("max-height", format!("{max_height}px")))
            }
            styles.push(("transform-origin", new_placement.transform_origin().to_string()));
            styles.push(("transform", format!(
                "translateX({left}px) translateY({top}px) {transform}"
            )));

            placement.set(new_placement);
        } else {
            logging::error!("Thaw-Binder: get_follower_placement_style return None");
        }

        styles.into_iter().for_each(|(name, value)| {
            content_ref.style().set_property(name, &value).unwrap_throw();
        });
    };

    let ensure_listener = move || {
        let target_ref = target_ref.get_untracked();
        let Some(el) = target_ref.as_deref() else {
            return;
        };

        let mut handle_vec = vec![];
        let mut cursor = get_scroll_parent_node(&el);
        loop {
            if let Some(node) = cursor.take() {
                cursor = get_scroll_parent_node(&node);

                let handle = add_event_listener(node, ev::scroll, move |_| {
                    sync_position();
                });
                handle_vec.push(handle);
            } else {
                break;
            }
        }
        scrollable_element_handle_vec.set_value(handle_vec);

        resize_handle.update_value(move |resize_handle| {
            if let Some(handle) = resize_handle.take() {
                handle.remove();
            }
            let handle = window_event_listener(ev::resize, move |_| {
                sync_position();
            });
            *resize_handle = Some(handle);
        });
    };

    let remove_listener = move || {
        scrollable_element_handle_vec.update_value(|vec| {
            vec.drain(..).for_each(|handle| handle.remove());
        });
        resize_handle.update_value(move |handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    };

    Effect::new(move |_| {
        if target_ref.get().is_none() {
            return;
        }
        if content_ref.get().is_none() {
            return;
        }
        if follower_show.get() {
            request_animation_frame(move || {
                sync_position();
            });

            remove_listener();
            ensure_listener();
        } else {
            remove_listener();
        }
    });

    Owner::on_cleanup(move || {
        remove_listener();
    });

    let follower_injection = FollowerInjection(Callback::new(move |_| sync_position()));

    UseBinder {
        target_ref,
        content_ref,
        follower_show,
        follower_ref,
        follower_injection,
        follower_children,
        placement,
    }
}
