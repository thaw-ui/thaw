# Nav Bar

```rust
use thaw::mobile::*;

let click_text = create_rw_signal(String::from("none"));
let on_click_left = move |_| click_text.set("left".to_string());
let on_click_right = move |_| click_text.set("right".to_string());

view! {
    <div style="height: 100vh;">
        <NavBar
            title="Home"
            left_arrow=true
            left_text="back"
            right_text="add"
            on_click_left=on_click_left
            on_click_right=on_click_right
        />
        <div style="padding-top: 50px">{move || click_text.get()}</div>
    </div>
}
```

### NavBar Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the NavBar element. |
| title | `MaybeSignal<String>` | `Default::default()` | NavBar title. |
| left_arrow | `MaybeSignal<bool>` | `false` | Whether to show left arrow. |
| left_text | `MaybeSignal<String>` | `Default::default()` | NavBar left text. |
| on_click_left | `Option<Callback<ev::MouseEvent>>` | `Default::default()` | NavBar left click. |
| right_text | `MaybeSignal<String>` | `Default::default()` | NavBar right text. |
| on_click_right | `Option<Callback<ev::MouseEvent>>` | `Default::default()` | NavBar right click. |
