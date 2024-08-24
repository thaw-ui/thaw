mod binder;
mod css_transition;
mod focus_trap;
mod if_comp;
mod option_comp;
mod teleport;

pub use binder::{Binder, Follower, FollowerInjection, FollowerPlacement, FollowerWidth};
pub use css_transition::CSSTransition;
pub use focus_trap::FocusTrap;
pub use if_comp::{ElseIf, If, Then};
pub use option_comp::OptionComp;
pub use teleport::Teleport;

use leptos::prelude::{slot, ChildrenFn};

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
