use super::FollowerPlacement;
use leptos::prelude::*;

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
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn refresh_position(&self) {
        self.0.run(());
    }
}
