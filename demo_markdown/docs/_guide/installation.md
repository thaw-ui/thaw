## Installation

Installation thaw

```shell
cargo add thaw --features=csr
```

## Usage

You just need to import thaw and use it.

```rust
// Import all
use thaw::*;
// Import on Demand
use thaw::{Button, ButtonAppearance};
```

A small example:

```rust
use leptos::prelude::*;
use thaw::*;

fn main() {
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Button appearance=ButtonAppearance::Primary>
            "Primary"
        </Button>
    }
}
```