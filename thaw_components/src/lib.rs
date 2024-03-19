mod binder;
mod css_transition;
mod if_comp;
mod option_comp;
mod teleport;
mod wave;

pub use binder::*;
pub use css_transition::CSSTransition;
pub use if_comp::*;
use leptos::*;
pub use option_comp::*;
pub use teleport::*;
pub use wave::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
