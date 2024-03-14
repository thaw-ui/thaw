# Internal component

There are some internal components that let developers know how to use them.

## Binder / Follwer

```rust
use crate::components::{Binder, Follower, FollowerPlacement};
use leptos::*;

// Used to internally track the location of this DOM
let div_ref= NodeRef::new();
// Used to turn on and off the position to listen to the DOM when the show changes
let show = RwSignal::new(false);

// placement: The position relative to the DOM when the popup opens

view! {
    <Binder target_ref=div_ref>
        <div ref=div_ref>
            "content"
        </div>
        <Follower slot show placement=FollowerPlacement::BottomStart>
            {
                move || {
                    show.get().then_some("popover")
                }
            }
        </Follower>
    </Binder>
}
```
