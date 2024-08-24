mod get_placement_style;

use std::ops::Deref;

pub use get_placement_style::FollowerPlacement;
use web_sys::wasm_bindgen::JsCast;

use crate::Teleport;
use get_placement_style::{get_follower_placement_offset, FollowerPlacementOffset};
use leptos::{
    context::Provider,
    ev,
    html::{self, ElementType},
    leptos_dom::helpers::WindowListenerHandle,
    prelude::*,
};
use thaw_utils::{add_event_listener, get_scroll_parent, mount_style, EventListenerHandle};

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
    /// The popup min width is the same as the target DOM width.
    MinTarget,
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
pub fn Binder<E>(
    /// Used to track DOM locations
    #[prop(into)]
    target_ref: NodeRef<E>,
    /// Content for pop-up display
    follower: Follower,
    children: Children,
) -> impl IntoView
where
    E: ElementType + 'static,
    E::Output: JsCast + Clone + Deref<Target = web_sys::HtmlElement> + 'static,
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
    let content_ref = NodeRef::<html::Div>::new();
    let content_style = RwSignal::new(String::new());
    let placement_str = RwSignal::new(follower_placement.as_str());
    let sync_position = move || {
        let Some(content_ref) = content_ref.get_untracked() else {
            return;
        };
        let Some(target_ref) = target_ref.get_untracked() else {
            return;
        };
        let target_rect = target_ref.get_bounding_client_rect();
        let content_rect = content_ref.get_bounding_client_rect();
        let mut style = String::new();
        if let Some(width) = follower_width {
            let width = match width {
                FollowerWidth::Target => format!("width: {}px;", target_rect.width()),
                FollowerWidth::MinTarget => format!("min-width: {}px;", target_rect.width()),
                FollowerWidth::Px(width) => format!("width: {width}px;"),
            };
            style.push_str(&width);
        }
        if let Some(FollowerPlacementOffset {
            top,
            left,
            transform,
            placement,
        }) = get_follower_placement_offset(follower_placement, target_rect, content_rect)
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
            error!("Thaw-Binder: get_follower_placement_style return None");
        }

        content_style.set(style);
    };

    let ensure_listener = move || {
        let target_ref = target_ref.get_untracked();
        let Some(el) = target_ref.as_deref() else {
            return;
        };

        let mut handle_vec = vec![];
        let mut cursor = get_scroll_parent(&el);
        loop {
            if let Some(el) = cursor.take() {
                cursor = get_scroll_parent(&el);

                let handle = add_event_listener(el, ev::scroll, move |_| {
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

    view! {
        {children()}
        <Teleport immediate=follower_show>
            <Provider value=follower_injection>
                <div class="thaw-binder-follower-container">
                    <div
                        class="thaw-binder-follower-content"
                        data-thaw-placement=move || placement_str.get()
                        node_ref=content_ref
                        style=move || content_style.get()
                    >
                        {follower_children()}
                    </div>
                </div>
            </Provider>
        </Teleport>
    }
}

#[derive(Debug, Clone)]
pub struct FollowerInjection(Callback<()>);

impl FollowerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn refresh_position(&self) {
        self.0.run(());
    }
}
