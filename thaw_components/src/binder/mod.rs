mod get_placement_style;

pub use get_placement_style::FollowerPlacement;

use crate::Teleport;
use get_placement_style::{get_follower_placement_offset, FollowerPlacementOffset};
use leptos::{html::ElementDescriptor, leptos_dom::helpers::WindowListenerHandle, *};
use thaw_utils::{
    add_event_listener, get_scroll_parent, mount_style, with_hydration_off, EventListenerHandle,
};

#[slot]
pub struct Follower {
    #[prop(into)]
    show: MaybeSignal<bool>,
    #[prop(optional)]
    width: Option<FollowerWidth>,
    #[prop(into)]
    placement: FollowerPlacement,
    children: Children,
}

#[derive(Clone)]
pub enum FollowerWidth {
    /// The popup width is the same as the target DOM width.
    Target,
    /// Customize the popup width.
    Px(u32),
}

impl Copy for FollowerWidth {}

/// # Tracking popup
///
/// ```rust
/// use crate::components::{Binder, Follower, FollowerPlacement};
/// use leptos::*;
///
/// let div_ref= NodeRef::new();
/// let show = RwSignal::new(false);
///
/// view! {
///     <Binder target_ref=div_ref>
///        <div ref=div_ref>
///            "content"
///        </div>
///        <Follower slot show placement=FollowerPlacement::BottomStart>
///           "content2"
///        </Follower>
///     </Binder>
/// }
/// ```
#[component]
pub fn Binder<El: ElementDescriptor + Clone + 'static>(
    /// Used to track DOM locations
    #[prop(into)]
    target_ref: NodeRef<El>,
    /// Content for pop-up display
    follower: Follower,
    children: Children,
) -> impl IntoView {
    mount_style("binder", include_str!("./binder.css"));
    let Follower {
        show: follower_show,
        width: follower_width,
        placement: follower_placement,
        children: follower_children,
    } = follower;

    let scroll_listener = store_value(None::<Callback<()>>);
    let scrollable_element_handle_vec = store_value::<Vec<EventListenerHandle>>(vec![]);
    let resize_handle = store_value(None::<WindowListenerHandle>);

    let ensure_scroll_listener = move || {
        let Some(el) = target_ref.get_untracked().map(|target| target.into_any()) else {
            return;
        };

        let mut handle_vec = vec![];
        let mut cursor = get_scroll_parent(&el);
        loop {
            if let Some(el) = cursor.take() {
                cursor = get_scroll_parent(&el);

                let handle = add_event_listener(el, ev::scroll, move |_| {
                    if let Some(scroll_listener) = scroll_listener.get_value() {
                        scroll_listener.call(());
                    }
                });
                handle_vec.push(handle);
            } else {
                break;
            }
        }
        scrollable_element_handle_vec.set_value(handle_vec);
    };

    let add_scroll_listener = Callback::new(move |listener: Callback<()>| {
        scroll_listener.update_value(|scroll_listener| {
            if scroll_listener.is_none() {
                ensure_scroll_listener();
            }
            *scroll_listener = Some(listener);
        })
    });

    let remove_scroll_listener = Callback::new(move |_| {
        scrollable_element_handle_vec.update_value(|vec| {
            vec.drain(..).for_each(|handle| handle.remove());
        });
        scroll_listener.set_value(None);
    });

    let add_resize_listener = Callback::new(move |listener: Callback<()>| {
        resize_handle.update_value(move |resize_handle| {
            if let Some(handle) = resize_handle.take() {
                handle.remove();
            }
            let handle = window_event_listener(ev::resize, move |_| {
                listener.call(());
            });

            *resize_handle = Some(handle);
        });
    });

    let remove_resize_listener = Callback::new(move |_| {
        resize_handle.update_value(move |handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        });
    });

    on_cleanup(move || {
        remove_scroll_listener.call(());
        remove_resize_listener.call(());
    });
    view! {
        {children()}
        <FollowerContainer
            show=follower_show
            target_ref
            width=follower_width
            placement=follower_placement
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
    width: Option<FollowerWidth>,
    placement: FollowerPlacement,
    #[prop(into)] add_scroll_listener: Callback<Callback<()>>,
    #[prop(into)] remove_scroll_listener: Callback<()>,
    #[prop(into)] add_resize_listener: Callback<Callback<()>>,
    #[prop(into)] remove_resize_listener: Callback<()>,
    children: Children,
) -> impl IntoView {
    let content_ref = create_node_ref::<html::Div>();
    let content_style = create_rw_signal(String::new());
    let placement_str = create_rw_signal(placement.as_str());
    let sync_position: Callback<()> = Callback::new(move |_| {
        let Some(content_ref) = content_ref.get_untracked() else {
            return;
        };
        let Some(target_ref) = target_ref.get_untracked().map(|target| target.into_any()) else {
            return;
        };
        let target_rect = target_ref.get_bounding_client_rect();
        let content_rect = content_ref.get_bounding_client_rect();
        let mut style = String::new();
        if let Some(width) = width {
            let width = match width {
                FollowerWidth::Target => format!("width: {}px;", target_rect.width()),
                FollowerWidth::Px(width) => format!("width: {width}px;"),
            };
            style.push_str(&width);
        }
        if let Some(FollowerPlacementOffset {
            top,
            left,
            transform,
            placement,
        }) = get_follower_placement_offset(placement, target_rect, content_rect)
        {
            placement_str.set(placement.as_str());
            style.push_str(&format!(
                "transform-origin: {};",
                placement.transform_origin()
            ));
            style.push_str(&format!(
                "transform: translateX({left}px) translateY({top}px) {transform};"
            ));
        } else {
            logging::error!("Thaw-Binder: get_follower_placement_style return None");
        }

        content_style.set(style);
    });

    Effect::new(move |_| {
        if target_ref.get().is_none() {
            return;
        }
        if content_ref.get().is_none() {
            return;
        }
        if show.get() {
            request_animation_frame(move || {
                sync_position.call(());
            });
            add_scroll_listener.call(sync_position);
            add_resize_listener.call(sync_position);
        } else {
            remove_scroll_listener.call(());
            remove_resize_listener.call(());
        }
    });

    let children = with_hydration_off(|| {
        html::div().classes("thaw-binder-follower-container").child(
            html::div()
                .classes("thaw-binder-follower-content")
                .attr("data-thaw-placement", move || placement_str.get())
                .node_ref(content_ref)
                .attr("style", move || content_style.get())
                .child(children()),
        )
    });

    view! { <Teleport element=children immediate=show/> }
}
