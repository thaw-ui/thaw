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

### Toast Intent

```rust demo
let toaster = ToasterInjection::expect_context();

fn dispatch_toast(toaster: ToasterInjection, intent: ToastIntent) {
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
            </ToastFooter>
        </Toast>
     }, ToastOptions::default().with_intent(intent));
};

view! {
    <Space>
        <Button on_click=move |_| dispatch_toast(toaster, ToastIntent::Info)>"Info"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastIntent::Success)>"Success"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastIntent::Warning)>"Warning"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastIntent::Error)>"Error"</Button>
    </Space>
}
```

### Dismiss Toast

```rust demo
let toaster = ToasterInjection::expect_context();
let id = uuid::Uuid::new_v4();


let mounted = RwSignal::new(false);

let on_status_change = move |status| {
    mounted.set(status == ToastStatus::Mounted);
};

let dispatch = move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Email sent"</ToastTitle>
            <ToastBody>
                "This is a toast body"
                <ToastBodySubtitle slot>
                    "Subtitle"
                </ToastBodySubtitle>
            </ToastBody>
        </Toast>
     },ToastOptions::default().with_id(id).with_on_status_change(on_status_change))
};

let dismiss = move |_| {
    toaster.dismiss_toast(id);
};

view! {
    {move || {if !mounted.get() {
        view!{<Button on_click=dispatch>"Show toast"</Button>}
    } else {
        view!{<Button on_click=dismiss>"Hide toast"</Button>}
    }
}}}
```


### Toast Title Media

```rust demo
let toaster = ToasterInjection::expect_context();

let on_click = move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>
                "Loading"
                <ToastTitleMedia slot>
                    <Spinner size=SpinnerSize::Tiny/>
                </ToastTitleMedia>
            </ToastTitle>
        </Toast>
     }, Default::default());
};

view! {
    <Button on_click=on_click>"Make toast"</Button>
}
```

### ToasterProvider Props

| Name     | Type            | Default                    | Description                           |
| -------- | --------------- | -------------------------- | ------------------------------------- |
| position | `ToastPosition` | `ToastPosition::BottomEnd` | The position the toast should render. |
| intent   | `ToastIntent  ` | `ToastPosition::Info`      | The intent of the toast.              |
| children | `Children`      |                            |                                       |

### ToastOptions Props

| Name          | Type                                    | Description                           |
| ------------- | --------------------------------------- | ------------------------------------- |
| with_position | `Fn(mut self, position: ToastPosition)` | The position the toast should render. |
| with_timeout  | `Fn(mut self, timeout: Duration)`       | Auto dismiss timeout in milliseconds. |
| with_intent   | `Fn(mut self, intent: ToastIntent)`     | The intent of the toast.              |

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
