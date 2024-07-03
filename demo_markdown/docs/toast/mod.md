# Toast

```rust demo
let toaster = ToasterInjection::use_();

let on_click = move |_| {
    toaster.dispatch_toast(view! { <span>"Hello"</span> }.into_any(), Default::default());
};

view! {
    <Button on_click=on_click>"Make toast"</Button>
}

```