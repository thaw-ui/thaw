use super::{
    get_placement_style::{get_follower_placement_offset, FollowerPlacementOffset},
    FollowerArrow, FollowerPlacement, FollowerWidth, UseBinder,
};
use leptos::{ev, html, leptos_dom::helpers::WindowListenerHandle, logging, prelude::*};
use std::sync::Arc;
use thaw_utils::{add_event_listener, get_scroll_parent_node, mount_style, EventListenerHandle};
use web_sys::wasm_bindgen::UnwrapThrowExt;

pub fn use_binder(
    follower_width: Option<FollowerWidth>,
    follower_placement: FollowerPlacement,
    auto_height: bool,
    arrow: Option<FollowerArrow>,
) -> UseBinder {
    mount_style("binder", include_str!("./binder.css"));

    let scrollable_element_handle_vec = StoredValue::<Vec<EventListenerHandle>>::new(vec![]);
    let resize_handle = StoredValue::new(None::<WindowListenerHandle>);
    let target_ref = NodeRef::<thaw_utils::Element>::new();
    let follower_ref = NodeRef::<html::Div>::new();
    let content_ref = NodeRef::<thaw_utils::HtmlElement>::new();
    let placement = RwSignal::new(follower_placement);
    let (arrow_safe_width, arrow_width, arrow_height, arrow_ref) =
        arrow.map_or((None, None, None, None), |arrow| {
            (
                Some(arrow.safe_width),
                Some(arrow.width),
                Some(arrow.height),
                Some(arrow.node_ref),
            )
        });

    let sync_position = move || {
        let Some(_) = follower_ref.get_untracked() else {
            return;
        };
        let Some(content_ref) = content_ref.get_untracked() else {
            return;
        };
        let Some(target_ref) = target_ref.get_untracked() else {
            return;
        };
        let target_rect = target_ref.get_bounding_client_rect();
        let content_rect = content_ref.get_bounding_client_rect();
        let mut styles = Vec::<(&str, String)>::new();
        styles.push(("position", "absolute".to_string()));
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
            &target_rect,
            &content_rect,
            arrow_height,
        ) {
            if auto_height {
                if let Some(max_height) = max_height {
                    styles.push(("max-height", format!("{max_height}px")))
                }
            }

            styles.push((
                "transform-origin",
                new_placement.transform_origin().to_string(),
            ));
            styles.push((
                "transform",
                format!("translateX({left}px) translateY({top}px) {transform}"),
            ));

            placement.set(new_placement);
        } else {
            logging::error!("Thaw-Binder: get_follower_placement_style return None");
        }

        styles.into_iter().for_each(|(name, value)| {
            content_ref
                .style()
                .set_property(name, &value)
                .unwrap_throw();
        });

        if let Some(arrow_el) = arrow_ref.map(|r| r.get_untracked()).flatten() {
            let style = (*arrow_el).style();
            let arrow_safe_width = arrow_safe_width.unwrap();
            let arrow_width = arrow_width.unwrap();
            let arrow_height = arrow_height.unwrap();
            let _ = style.remove_property("left");
            let _ = style.remove_property("top");

            match placement.get_untracked() {
                FollowerPlacement::Top | FollowerPlacement::Bottom => {
                    let _ = style.set_property(
                        "left",
                        &format!(
                            "calc({}px + var(--thaw-positioning-arrow-offset))",
                            content_rect.width() / 2.0
                        ),
                    );
                }
                FollowerPlacement::TopStart | FollowerPlacement::BottomStart => {
                    let content_width = content_rect.width();
                    let target_width = target_rect.width();
                    let target_width_half = target_width / 2.0;
                    if content_width > target_width && target_width_half < arrow_width * 3.0 {
                        let left = (target_width_half - arrow_width).max(arrow_safe_width);
                        let _ = style.set_property("left", &format!("{}px", left));
                    } else {
                        let _ = style.set_property(
                            "left",
                            "calc(var(--thaw-positioning-arrow-offset) * -2)",
                        );
                    }
                }
                FollowerPlacement::TopEnd | FollowerPlacement::BottomEnd => {
                    let content_width = content_rect.width();
                    let target_width = target_rect.width();
                    let target_width_half = target_width / 2.0;
                    if content_width > target_width && target_width_half < arrow_width * 3.0 {
                        let right = (target_width_half - arrow_width).max(arrow_safe_width);
                        let _ = style.set_property("right", &format!("{}px", right));
                    } else {
                        let _ = style.set_property(
                            "right",
                            "calc(var(--thaw-positioning-arrow-offset) * -2)",
                        );
                    }
                }
                FollowerPlacement::Left | FollowerPlacement::Right => {
                    let _ = style.set_property(
                        "top",
                        &format!(
                            "calc({}px + var(--thaw-positioning-arrow-offset))",
                            content_rect.height() / 2.0
                        ),
                    );
                }
                FollowerPlacement::LeftStart | FollowerPlacement::RightStart => {
                    let content_height = content_rect.height();
                    let target_height = target_rect.height();
                    let target_height_half = target_height / 2.0;
                    if content_height > target_height && target_height_half < arrow_width * 3.0 {
                        let top = (target_height_half - arrow_width).max(arrow_safe_width);
                        let _ = style.set_property("top", &format!("{}px", top));
                    } else {
                        let _ = style
                            .set_property("top", "calc(var(--thaw-positioning-arrow-offset) * -2)");
                    }
                }
                FollowerPlacement::LeftEnd | FollowerPlacement::RightEnd => {
                    let content_height = content_rect.height();
                    let target_height = target_rect.height();
                    let target_height_half = target_height / 2.0;
                    if content_height > target_height && target_height_half < arrow_height * 3.0 {
                        let bottom = (target_height_half - arrow_width).max(arrow_safe_width);
                        let _ = style.set_property("bottom", &format!("{}px", bottom));
                    } else {
                        let _ = style.set_property(
                            "bottom",
                            "calc(var(--thaw-positioning-arrow-offset) * -2)",
                        );
                    }
                }
            }
        }
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

    Owner::on_cleanup(move || {
        remove_listener();
    });

    UseBinder {
        target_ref,
        content_ref,
        follower_ref,
        placement,
        sync_position: Arc::new(sync_position),
        ensure_listener: Arc::new(ensure_listener),
        remove_listener: Arc::new(remove_listener),
    }
}
