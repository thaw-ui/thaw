# Usage

You just need to import thaw and use it.

```rust
// Import all
use thaw::*;
// Import on Demand
use thaw::{Button, ButtonVariant};
```

A small example:

```rust
use leptos::*;
use thaw::*;

fn main() {
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Button variant=ButtonVariant::Primary>"Primary"</Button>
    }
}
```
