# Radio

```rust demo
let value = create_rw_signal(false);

view! {
    <Radio value>"Click"</Radio>
}
```

### Group

```rust demo
let value = create_rw_signal(None);

view! {
    <RadioGroup value>
        <RadioItem key="a">
            "Apple"
        </RadioItem>
        <RadioItem key="o">
            "Orange"
        </RadioItem>
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
