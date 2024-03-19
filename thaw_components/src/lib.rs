mod binder;
mod css_transition;
mod if_comp;
mod option_comp;
mod teleport;
mod wave;

pub use binder::{Binder, Follower, FollowerPlacement, FollowerWidth};
pub use css_transition::CSSTransition;
pub use if_comp::{ElseIf, If, Then};
pub use option_comp::OptionComp;
pub use teleport::Teleport;
pub use wave::{Wave, WaveRef};

use leptos::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
