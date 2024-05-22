# SpinButton

```rust demo
let value = RwSignal::new(0);
let value_f64 = RwSignal::new(0.0);

view! {
    <Space vertical=true>
        <SpinButton value step_page=1/>
        <SpinButton value=value_f64 step_page=1.2/>
    </Space>
}
```

### Min / Max

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton value step_page=1 min=-1 max=2/>
}
```

### Disabled

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton value step_page=1 disabled=true/>
}
```