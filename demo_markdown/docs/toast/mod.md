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
let toaster = ToasterInjection::expect_context();

fn dispatch_toast(toaster: ToasterInjection, position: ToastPosition) {
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
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::BottomStart)>"BottomStart"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::BottomEnd)>"BottomEnd"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::Top)>"Top"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::TopStart)>"Topstart"</Button>
        <Button on_click=move |_| dispatch_toast(toaster, ToastPosition::TopEnd)>"TopEnd"</Button>
    </Space>
}
```
