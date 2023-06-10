mod if_comp;
mod option_comp;

pub use if_comp::*;
pub use option_comp::*;
use leptos::*;

#[slot]
pub struct Fallback {
    children: ChildrenFn,
}
