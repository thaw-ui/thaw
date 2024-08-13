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

### Radio Props

| Name  | Type                | Default              | Description                                         |
| ----- | ------------------- | -------------------- | --------------------------------------------------- |
| class | `MaybeProp<String>` | `Default::default()` |                                                     |
| value | `String`            | `Default::default()` | The value of the radio to be used in a radio group. |
| label | `MaybeProp<String>` | `None`               | The Radio's label.                                  |

### RadioGroup Props

| Name     | Type                  | Default              | Description                            |
| -------- | --------------------- | -------------------- | -------------------------------------- |
| class    | `MaybeProp<String>`   | `Default::default()` |                                        |
| value    | `OptionModel<String>` | `Default::default()` | The selected Radio item in this group. |
| name     | `Option<String>`      | `None`               | The name of this radio group.          |
| children | `Children`            |                      |                                        |
