# Radio

```rust demo
let value = RwSignal::new(None);

view! {
    <RadioGroup value>
        <Radio value="a" label="Apple"/>
        <Radio value="o" label="Orange"/>
    </RadioGroup>
    <div style="margin-top: 1rem">
        "value: " {move || format!("{:?}", value.get())}
    </div>
}
```

### Radio Props

| Name     | Type                                | Default              | Description                              |
| -------- | ----------------------------------- | -------------------- | ---------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the radio element. |
| value    | `Model<bool>`                       | `false`              | Checked value.                           |
| children | `Option<Children>`                  | `None`               | Radio's content.                         |

### RadioGroup Props

| Name     | Type                    | Default              | Description                        |
| -------- | ----------------------- | -------------------- | ---------------------------------- |
| value    | `Model<Option<String>>` | `Default::default()` | Sets the value of the radio group. |
| children | `Children`              |                      | RadioGroup's content.              |

### RadioItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the radio element. |
| key | `String` |  | The key of the radio to be used in a radio group. |
| children | `Option<Children>` | `None` | Radio's content. |
