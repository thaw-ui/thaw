# Radio

```rust demo
let value = create_rw_signal(false);

view! {
    <Radio value>"Click"</Radio>
}
```

### Radio Props

| Name     | Type                                | Default              | Description                              |
| -------- | ----------------------------------- | -------------------- | ---------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the radio element. |
| value    | `Model<bool>`                       | `false`              | Checked value.                           |
| children | `Children`                          |                      | Radio's content.                         |
