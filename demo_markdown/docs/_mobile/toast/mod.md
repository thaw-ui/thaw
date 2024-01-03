# Toast

```rust
use thaw::mobile::*;
use std::time::Duration;

let count = create_rw_signal(0u32);
let onclick = move |_| {
    show_toast(ToastOptions {
        message: format!("Hello {}", count.get_untracked()),
        duration: Duration::from_millis(2000),
    });
    count.set(count.get_untracked() + 1);
};
view! {
    <div style="margin: 20px">
        <Button on_click=onclick>"hi"</Button>
    </div>
}
```

### Toast Methods

| Name       | Type                         | Description |
| ---------- | ---------------------------- | ----------- |
| show_toast | `Fn(options: ToastOptions))` | Show toast. |

### ToastOptions Properties

| Name     | Type                  | Description    |
| -------- | --------------------- | -------------- |
| message  | `String`              | message.       |
| duration | `std::time::Duration` | Show duration. |
