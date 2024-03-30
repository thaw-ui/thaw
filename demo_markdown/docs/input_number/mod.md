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

### Min / Max

```rust demo
let value = create_rw_signal(0);

view! {
    <InputNumber value step=1 min=-1 max=2/>
}
```

### InputNumber Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the input element. |
| value | `Model<T>` | `T::default()` | Set the input value. |
| placeholder | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Placeholder of input number. |
| step | `MaybeSignal<T>` |  | The number which the current value is increased or decreased on key or button press. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| invalid | `MaybeSignal<bool>` | `false` | Whether the input is invalid. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |
| min | `MaybeSignal<T>` | `T::min_value()` | The minimum number that the input value can take. |
| max | `MaybeSignal<T>` | `T::max_value()` | The max number that the input value can take. |

#### T impl

`T: Add<Output = T> + Sub<Output = T> + PartialOrd + num_traits::Bounded + Default + Clone + FromStr + ToString + 'static`

### InputNumber Ref

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
