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

### Dialog Props

| Name           | Type                | Default              | Description                                 |
| -------------- | ------------------- | -------------------- | ------------------------------------------- |
| class          | `MaybeProp<String>` | `Default::default()` |                                             |
| open           | `Model<bool>`       |                      | Controls the open state of the dialog.      |
| mask_closeable | `Signal<bool>`      | `true`               | Whether to emit hide event when click mask. |
| close_on_esc   | `bool`              | `true`               | Whether to close modal on Esc is pressed.   |
| children       | `Children`          |                      |                                             |

### DialogSurface Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### DialogBody Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### DialogTitle Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### DialogContent Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### DialogActions Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
