# Input Number

```rust demo
let value = create_rw_signal(0);
let value_f64 = create_rw_signal(0.0);

view! {
    <Space vertical=true>
        <InputNumber value step=1/>
        <InputNumber value=value_f64 step=1.0/>
    </Space>
}
```

### Disabled

```rust demo
let value = create_rw_signal(0);

view! {
    <InputNumber value step=1 disabled=true/>
}
```

### Invalid

```rust demo
let value = create_rw_signal(0);

view! {
    <InputNumber value step=1 invalid=true/>
}
```

### InputNumber Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the input element. |
| value | `RwSignal<T>` | `T::default()` | Set the input value. |
| placeholder | `MaybeSignal<String>` | `Default::default()` | Placeholder of input number. |
| step | `MaybeSignal<T>` |  | The number which the current value is increased or decreased on key or button press. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| invalid | `MaybeSignal<bool>` | `false` | Whether the input is invalid. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |

#### T impl

`T: Add<Output = T> + Sub<Output = T> + Default + Clone + FromStr + ToString + 'static`
