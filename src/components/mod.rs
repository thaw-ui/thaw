mod binder;
mod if_comp;
mod option_comp;

pub use if_comp::*;
use leptos::*;
pub use option_comp::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
