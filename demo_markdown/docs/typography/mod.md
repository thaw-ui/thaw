# Typography

```rust demo
view! {
    <Space>
        <Text>"text"</Text>
        <Text code=true>"code"</Text>
    </Space>
}
```

## Text Props

| Name     | Type                                | Default              | Description                             |
| -------- | ----------------------------------- | -------------------- | --------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the text element. |
| code     | `bool`                              | `false`              | Use the code tag and style.             |
| children | `Children`                          |                      | Text's content.                         |
