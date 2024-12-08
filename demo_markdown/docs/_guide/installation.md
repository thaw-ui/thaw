## Installation

Installation thaw.

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

<MessageBar intent=MessageBarIntent::Warning attr:style="margin-top: 12px">
    <MessageBarBody>
        <div style="white-space: normal">
            "Currently Thaw supports CSR and SSR. Island is not supported yet."
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
        // The ConfigProvider component is required for Thaw,
        // please place it at the root of the Thaw component.
        <ConfigProvider>
            <Button appearance=ButtonAppearance::Primary>
                "Primary"
            </Button>
        </ConfigProvider>
    }
}
```