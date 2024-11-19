# SpinButton

SpinButton are used to allow numerical input bounded between minimum and maximum values with buttons to increment and decrement the input value.

<MessageBar intent=MessageBarIntent::Warning>
    <MessageBarBody>
        <div style="white-space: normal">
            "SpinButton is a generic component, so the type must be specified. Example: <SpinButton<i32> step_page=1/>. "
            <Link href="https://github.com/leptos-rs/leptos/issues/3200">Related issue</Link>
        </div>
    </MessageBarBody>
</MessageBar>

```rust demo
let value = RwSignal::new(0);
let value_f64 = RwSignal::new(0.0);

view! {
    <Space vertical=true>
        <SpinButton<i32> value step_page=1/>
        <SpinButton<f64> value=value_f64 step_page=1.2/>
    </Space>
}
```

### Min / Max

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton<i32> value step_page=1 min=-1 max=2/>
}
```

### Disabled

```rust demo
let value = RwSignal::new(0);

view! {
    <SpinButton<i32> value step_page=1 disabled=true/>
}
```

### Custom parsing

```rust demo
let value = RwSignal::new(0.0);

let format = move |v: f64| {
    let v = v.to_string();
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
};

let parser = move |v: String| {
    let comma_pos = v.chars().position(|c| c == ',').unwrap_or_else(|| v.chars().count());
    let int_part = v.chars().take(comma_pos).filter(|a| a.is_digit(10)).collect::<String>();
    let dec_part = v.chars().skip(comma_pos + 1).take(2).filter(|a| a.is_digit(10)).collect::<String>();

    format!("{:0<1}.{:0<2}", int_part, dec_part).parse::<f64>().ok()
};

view! {
    <SpinButton<f64> value parser format step_page=1.0 />
    <p>"Underlying value: "{ value }</p>
}
```

### SpinButton Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<SpinButtonRule<T>>` | `vec![]` | The rules to validate Field. |
| value | `Model<T>` | `T::default()` | Current value of the control. |
| placeholder | `MaybeProp<String>` | `Default::default()` | Placeholder of input number. |
| step_page | `Signal<T>` |  | Large difference between two values. This should be greater than step and is used when users hit the Page Up or Page Down keys. |
| min | `Signal<T>` | `T::min_value()` | The minimum number that the input value can take. |
| max | `Signal<T>` | `T::max_value()` | The maximum number that the input value can take. |
| disabled | `Signal<bool>` | `false` | Whether the input is disabled. |
| parser | `OptionalProp<BoxOneCallback<String, Option<T>>>` | `None` | Modifies the user input before assigning it to the value. |
| format | `OptionalProp<BoxOneCallback<T, String>>` | `None` | Formats the value to be shown to the user. |

#### T impl

```rust
where
    T: Send + Sync,
    T: Add<Output = T> + Sub<Output = T> + PartialOrd + Bounded,
    T: Default + Clone + FromStr + ToString + 'static,
```
