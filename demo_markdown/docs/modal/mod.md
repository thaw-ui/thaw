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

| Name           | Type                  | Default              | Description                                 |
| -------------- | --------------------- | -------------------- | ------------------------------------------- |
| class          | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the modal element. |
| show           | `Model<bool>`         |                      | Whether to show modal.                      |
| title          | `MaybeSignal<String>` | `Default::default()` | Modal title.                                |
| width          | `MaybeSignal<String>` | `600px`              | Modal width.                                |
| z_index        | `MaybeSignal<i16>`    | `2000`               | z-index of the modal.                       |
| mask_closeable | `MaybeSignal<bool>`   | `true`               | Whether to emit hide event when click mask. |
| close_on_esc   | `bool`                | `true`               | Whether to close modal on Esc is pressed.   |
| closable       | `bool`                | `true`               | Whether to display the close button.        |
| children       | `Children`            |                      | Modal's content.                            |

### Modal Slots

| Name        | Default | Description     |
| ----------- | ------- | --------------- |
| ModalFooter | `None`  | Footer content. |
