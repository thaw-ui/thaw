mod get_placement_style;
mod types;
pub mod use_binder;

pub use get_placement_style::FollowerPlacement;
pub use types::*;
use use_binder::{use_binder, UseBinder};

use crate::Teleport;
use leptos::{
    context::Provider, prelude::*,
    tachys::html::node_ref::node_ref,
};

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
pub fn Binder<T, FT>(
    /// Content for pop-up display
    follower: Follower<FT>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
    FT: AddAnyAttr + IntoView + Send + 'static,
{
    let UseBinder {
        target_ref,
        content_ref, 
        follower_show,
        follower_ref,
        follower_children, follower_injection, placement } = use_binder(follower);

    view! {
        {children.into_inner()().into_inner().add_any_attr(node_ref(target_ref))}
        <Teleport immediate=follower_show>
            <div class="thaw-binder-follower" node_ref=follower_ref data-thaw-placement=move || placement.get().as_str()>
                <Provider value=follower_injection>
                    {follower_children
                        .into_inner()().into_inner()
                        .add_any_attr(node_ref(content_ref))}
                </Provider>
            </div>
        </Teleport>
    }
}
