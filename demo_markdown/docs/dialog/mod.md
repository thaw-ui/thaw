# Dialog

```rust demo
let open = RwSignal::new(false);

view! {
    <Button on_click=move |_| open.set(true)>"Open Dialog"</Button>
    <Dialog open>
        <DialogSurface>
            <DialogBody>
                <DialogTitle>"Dialog title"</DialogTitle>
                <DialogContent>
                    "Dialog body"
                </DialogContent>
                <DialogActions>
                    <Button appearance=ButtonAppearance::Primary>"Do Something"</Button>
                </DialogActions>
            </DialogBody>
        </DialogSurface>
    </Dialog>
}
```

### Modal Props

| Name           | Type                  | Default              | Description                                 |
| -------------- | --------------------- | -------------------- | ------------------------------------------- |
| show           | `Model<bool>`         |                      | Whether to show modal.                      |
| title          | `MaybeSignal<String>` | `Default::default()` | Modal title.                                |
| width          | `MaybeSignal<String>` | `600px`              | Modal width.                                |
| z_index        | `MaybeSignal<i16>`    | `2000`               | z-index of the modal.                       |
| mask_closeable | `MaybeSignal<bool>`   | `true`               | Whether to emit hide event when click mask. |
| close_on_esc   | `bool`                | `true`               | Whether to close modal on Esc is pressed.   |
| children       | `Children`            |                      | Modal's content.                            |

### Modal Slots

| Name        | Default | Description     |
| ----------- | ------- | --------------- |
| ModalFooter | `None`  | Footer content. |
