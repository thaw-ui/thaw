# SpinButton

```rust demo
let value = RwSignal::new(0);
let value_f64 = RwSignal::new(0.0);

view! {
    <Space vertical=true>
        <SpinButton value step_page=1/>
        <SpinButton value=value_f64 step_page=1.0/>
    </Space>
}
```