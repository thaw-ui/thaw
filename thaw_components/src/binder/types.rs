use super::FollowerPlacement;
use leptos::{html, prelude::*};
use std::sync::Arc;

#[slot]
pub struct Follower<T>
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    #[prop(into)]
    show: Signal<bool>,
    #[prop(optional)]
    width: Option<FollowerWidth>,
    #[prop(into)]
    placement: FollowerPlacement,
    children: TypedChildren<T>,
    #[prop(optional)]
    auto_height: bool,
    #[prop(optional)]
    arrow: Option<FollowerArrow>,
}

#[derive(Debug, Clone)]
pub enum FollowerWidth {
    /// The popup width is the same as the target DOM width.
    Target,
    /// The popup min width is the same as the target DOM width.
    MinTarget,
    /// Customize the popup width.
    Px(u32),
}

impl Copy for FollowerWidth {}

#[derive(Debug, Clone)]
pub struct FollowerInjection(pub(crate) Callback<()>);

impl FollowerInjection {
    pub fn new(f: impl Fn() + Send + Sync + 'static) -> Self {
        Self(Callback::new(move |_| f()))
    }
}

impl FollowerInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn refresh_position(&self) {
        self.0.run(());
    }
}

pub struct FollowerArrow {
    pub safe_width: f64,
    pub width: f64,
    pub height: f64,
    pub node_ref: NodeRef<html::Div>,
}

pub struct UseBinder {
    pub target_ref: NodeRef<thaw_utils::Element>,
    pub content_ref: NodeRef<thaw_utils::HtmlElement>,
    pub follower_ref: NodeRef<html::Div>,
    pub placement: RwSignal<FollowerPlacement>,
    pub sync_position: Arc<dyn Fn() -> () + Send + Sync>,
    pub ensure_listener: Arc<dyn Fn() -> () + Send>,
    pub remove_listener: Arc<dyn Fn() -> () + Send>,
}
