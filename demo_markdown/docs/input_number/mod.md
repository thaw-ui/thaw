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

let formatter = Callback::<String, String>::new(move |v: String| {
    let mut int: String = if v.chars().count() < 4 { 
        String::from("0") 
    } else {
        v.chars().take(v.chars().count() - 3).collect() 
    };

    let sign: String = if v.chars().take(1).collect::<String>() == String::from("-") { 
        int = int.chars().skip(1).collect();
        String::from("-")
    } else { 
        String::from("") 
    };

    let dec: String = if v.chars().count() < 2 {
        format!("{:0>2}" , v)
    } else {
        v.chars().skip(v.chars().count() - 2).take(2).collect()
    };

    let int = int
        .as_bytes()
        .rchunks(3)
        .rev()
        .map(std::str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap()
        .join("."); // separator
    format!("{}{},{:0>2}", sign, int, dec)
});

let parser = Callback::<String, String>::new(move |v: String| {
    let digits = v.chars().filter(|a| a.is_digit(10)).collect::<String>();
    let float = digits.parse::<f64>().unwrap_or_else(|_| 1.0);
    format!("{:.2}", float / 100.0)
});

view! {
    <InputNumber value parser formatter step=0.01 />
    <p>{ value }</p>
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

#### T impl

`T: Add<Output = T> + Sub<Output = T> + PartialOrd + num::Bounded + Default + Clone + FromStr + ToString + 'static`

### InputNumber Ref

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
