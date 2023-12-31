# Modal

```rust demo
let show = create_rw_signal(false);

view! {
    <Button on_click=move |_| show.set(true)>"Open Modal"</Button>
    <Modal title="title" show>
        "hello"
    </Modal>
}
```

### Modal Props

| Name     | Type                  | Default              | Description            |
| -------- | --------------------- | -------------------- | ---------------------- |
| show     | `MaybeSignal<bool>`   |                      | Whether to show modal. |
| title    | `MaybeSignal<String>` | `Default::default()` | Modal title.           |
| children | `Children`            |                      | Modal's content.       |

### Modal Slots

| Name        | Default | Description     |
| ----------- | ------- | --------------- |
| ModalFooter | `None`  | Footer content. |
