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

let dispatch_toast = Callback::new(move |position| {
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
});

view! {
    <Space>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::BottomStart)>"BottomStart"</Button>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::BottomEnd)>"BottomEnd"</Button>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::Top)>"Top"</Button>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::TopStart)>"Topstart"</Button>
        <Button on_click=move |_| dispatch_toast.call(ToastPosition::TopEnd)>"TopEnd"</Button>
    </Space>
}
```
