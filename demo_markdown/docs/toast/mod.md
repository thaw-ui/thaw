# Toast

```rust demo
let toaster = ToasterInjection::expect_context();

let on_click = move |_| {
    toaster.dispatch_toast(view! {
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
     }.into_any(), Default::default());
};

view! {
    <Button on_click=on_click>"Make toast"</Button>
}
```

### Toast Positions

```rust demo
fn dispatch_toast(position: ToastPosition) {
    let toaster = ToasterInjection::expect_context();

    toaster.dispatch_toast(view! {
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
     }.into_any(), ToastOptions::default().with_position(position));
};

view! {
    <Space>
        <Button on_click=move |_| dispatch_toast(ToastPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| dispatch_toast(ToastPosition::BottomStart)>"BottomStart"</Button>
        <Button on_click=move |_| dispatch_toast(ToastPosition::BottomEnd)>"BottomEnd"</Button>
        <Button on_click=move |_| dispatch_toast(ToastPosition::Top)>"Top"</Button>
        <Button on_click=move |_| dispatch_toast(ToastPosition::TopStart)>"Topstart"</Button>
        <Button on_click=move |_| dispatch_toast(ToastPosition::TopEnd)>"TopEnd"</Button>
    </Space>
}
```
