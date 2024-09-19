## Installation

Installation thaw

```shell
cargo add thaw --features=csr
```

<MessageBar intent=MessageBarIntent::Warning>
    <MessageBarBody>
        <div style="white-space: normal">
            "If you are using the nightly feature in Leptos, please enable Thaw's nightly as well."
        </div>
    </MessageBarBody>
</MessageBar>

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
        <ConfigProvider>
            <Button appearance=ButtonAppearance::Primary>
                "Primary"
            </Button>
        </ConfigProvider>
    }
}
```