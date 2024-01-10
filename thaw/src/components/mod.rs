mod binder;
mod css_transition;
mod if_comp;
mod option_comp;
mod teleport;
mod wave;

pub(crate) use binder::*;
pub(crate) use css_transition::CSSTransition;
pub(crate) use if_comp::*;
use leptos::*;
pub(crate) use option_comp::*;
pub(crate) use teleport::*;
pub(crate) use wave::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
