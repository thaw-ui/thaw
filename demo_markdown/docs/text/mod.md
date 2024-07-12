# Typography

```rust demo
view! {
    <Space>
        <Text>"text"</Text>
        <Text tag=TextTag::Code>"code"</Text>
        <Caption1>"Caption1"</Caption1>
        <Caption1Strong>"Caption1Strong"</Caption1Strong>
        <Body1>"Body1"</Body1>
    </Space>
}
```

## Text Props

| Name     | Type                                | Default              | Description                             |
| -------- | ----------------------------------- | -------------------- | --------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the text element. |
| code     | `bool`                              | `false`              | Use the code tag and style.             |
| children | `Children`                          |                      | Text's content.                         |
