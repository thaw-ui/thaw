# Toast

```rust demo
let toaster = ToasterInjection::expect_context();

let on_click = move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Email sent"</ToastTitle>
            <ToastBody>
                "This is a toast body"
                <ToastBodySubtitle slot>
                    "Subtitle"
                </ToastBodySubtitle>
            </ToastBody>
            <ToastFooter>
                "Footer"
                // <Link>Action</Link>
                // <Link>Action</Link>
            </ToastFooter>
        </Toast>
     }, Default::default());
};

view! {
    <Button on_click=on_click>"Make toast"</Button>
}
```

### Toast Positions

```rust demo
let toaster = ToasterInjection::expect_context();

fn dispatch_toast(toaster: ToasterInjection, position: ToastPosition) {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Email sent"</ToastTitle>
            <ToastBody>
                "This is a toast body"
                <ToastBodySubtitle slot>
                    "Subtitle"
                </ToastBodySubtitle>
            </ToastBody>
            <ToastFooter>
                "Footer"
                // <Link>Action</Link>
                // <Link>Action</Link>
            </ToastFooter>
        </Toast>
     }, ToastOptions::default().with_position(position));
};

view! {
    <Space>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::BottomStart)>"BottomStart"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::BottomEnd)>"BottomEnd"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::Top)>"Top"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::TopStart)>"Topstart"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::TopEnd)>"TopEnd"</Button>
    </Space>
}
```

### ToasterProvider Props

| Name     | Type            | Default                    | Description                           |
| -------- | --------------- | -------------------------- | ------------------------------------- |
| position | `ToastPosition` | `ToastPosition::BottomEnd` | The position the toast should render. |
| children | `Children`      |                            |                                       |

### ToastOptions Props

| Name          | Type                                    | Description                           |
| ------------- | --------------------------------------- | ------------------------------------- |
| with_position | `Fn(mut self, position: ToastPosition)` | The position the toast should render. |
| with_timeout  | `Fn(mut self, timeout: Duration)`       | Auto dismiss timeout in milliseconds. |
| with_intent   | `Fn(mut self, intent: ToastIntent)`     | Intent.                               |

### Toast & ToastFooter Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### ToastTitle Props

| Name               | Type                            | Default | Description |
| ------------------ | ------------------------------- | ------- | ----------- |
| toast_title_media  | slot `Option<ToastTitleMedia>`  | `None`  |             |
| toast_title_action | slot `Option<ToastTitleAction>` | `None`  |             |
| children           | `Children`                      |         |             |

### ToastTitleMedia & ToastTitleAction Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### ToastBody Props

| Name                | Type                             | Default | Description |
| ------------------- | -------------------------------- | ------- | ----------- |
| toast_body_subtitle | slot `Option<ToastBodySubtitle>` | `None`  |             |
| children            | `Children`                       |         |             |

### ToastBodySubtitle Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |
