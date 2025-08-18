# Radio

```rust demo
let value = RwSignal::new(String::new());
let option_value = RwSignal::new(None);

view! {
    <Space vertical=true>
        <RadioGroup value>
            <Radio value="a" label="Apple"/>
            <Radio value="o" label="Orange"/>
        </RadioGroup>
        <RadioGroup value=option_value>
            <Radio value="a" label="Apple"/>
            <Radio value="o" label="Orange"/>
        </RadioGroup>
    </Space>
    <div style="margin-top: 1rem">
        "value: " {move || format!("{}", value.get())}
    </div>
    <div style="margin-top: 1rem">
        "option_value: " {move || format!("{:?}", option_value.get())}
    </div>
}
```

## Disabled

RadioGroup can be disabled, which disables all Radio items inside.

```rust demo
view! {
    <RadioGroup value="a".to_string() disabled=true>
        <Radio value="a" label="Apple"/>
        <Radio value="o" label="Orange"/>
    </RadioGroup>
}
```

### Radio Props

| Name  | Type                | Default              | Description                                         |
| ----- | ------------------- | -------------------- | --------------------------------------------------- |
| class | `MaybeProp<String>` | `Default::default()` |                                                     |
| value | `String`            | `Default::default()` | The value of the radio to be used in a radio group. |
| label | `MaybeProp<String>` | `None`               | The Radio's label.                                  |

### RadioGroup Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| disabled | `Signal<bool>` | `false` | Disable all Radio items in this group. |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<RadioGroupRule>` | `vec![]` | The rules to validate Field. |
| value | `OptionModel<String>` | `Default::default()` | The selected Radio item in this group. |
| name | `MaybeProp<String>` | `None` | The name of this radio group. |
| children | `Children` |  |  |
