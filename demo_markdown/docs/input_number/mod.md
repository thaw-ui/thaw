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

### Min / Max

```rust demo
let value = create_rw_signal(0);

view! {
    <InputNumber value step=1 min=-1 max=2/>
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

### Formatter

```rust demo
let value = create_rw_signal(0.0);
let value_2 = create_rw_signal(0.0);

let formatter = Callback::<String, String>::new(move |v: String| {
    let dot_pos = v.chars().position(|c| c == '.').unwrap_or_else(|| v.chars().count());
    let mut int: String = v.chars().take(dot_pos).collect();

    let sign: String = if v.chars().take(1).collect::<String>() == String::from("-") { 
        int = int.chars().skip(1).collect();
        String::from("-")
    } else { 
        String::from("") 
    };

    let dec: String = v.chars().skip(dot_pos + 1).take(2).collect();

    let int = int
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join(".");
    format!("{}{},{:0<2}", sign, int, dec)
});

let parser = Callback::<String, String>::new(move |v: String| {
    let comma_pos = v.chars().position(|c| c == ',').unwrap_or_else(|| v.chars().count());
    let int_part = v.chars().take(comma_pos).filter(|a| a.is_digit(10)).collect::<String>();
    let dec_part = v.chars().skip(comma_pos + 1).take(2).filter(|a| a.is_digit(10)).collect::<String>();
    format!("{:0<1}.{:0<2}", int_part, dec_part)
});

view! {
    <InputNumber value parser formatter step=1.0 />
    <p>"Underlying value: "{ value }</p>
    <InputNumber value=value_2 parser formatter step=1.0 />
    <p>"Underlying value_2: "{ value_2 }</p>
}
```
### InputNumber Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the input element. |
| value | `Model<T>` | `T::default()` | Set the input value. |
| placeholder | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Placeholder of input number. |
| step | `MaybeSignal<T>` |  | The number which the current value is increased or decreased on key or button press. |
| min | `MaybeSignal<T>` | `T::min_value()` | The minimum number that the input value can take. |
| max | `MaybeSignal<T>` | `T::max_value()` | The maximum number that the input value can take. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| invalid | `MaybeSignal<bool>` | `false` | Whether the input is invalid. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |
| parser | `OptionalProp<Callback<String, String>>` | `Default::default()` | Modifies the user input before assigning it to the value |
| formatter | `OptionalProp<Callback<String, String>>` | `Default::default()` | Formats the value to be shown to the user |

#### T impl

`T: Add<Output = T> + Sub<Output = T> + PartialOrd + num::Bounded + Default + Clone + FromStr + ToString + 'static`

### InputNumber Ref

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
