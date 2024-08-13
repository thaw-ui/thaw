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

## Text & Caption1 & Caption1Strong & Body1 & Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| style    | `MaybeProp<String>` | `Default::default()` |             |
| tag      | `TextTag`           | `TextTag::Span`      |             |
| children | `Children`          |                      |             |
