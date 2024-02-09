# Alert

```rust demo
view! {
    <Space vertical=true>
        <Alert variant=AlertVariant::Success title="title">
            "success"
        </Alert>
        <Alert variant=AlertVariant::Warning title="title">
            "warning"
        </Alert>
        <Alert variant=AlertVariant::Error title="title">
            "error"
        </Alert>
    </Space>
}
```

### Alert Props

| Name     | Type                                | Default              | Description                               |
| -------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Additional classes for the alert element. |
| title    | `Option<MaybeSignal<String>>`       | `Default::default()` | Title of the alert.                       |
| variant  | `MaybeSignal<AlertVariant>`         |                      | Alert variant.                            |
| children | `Children`                          |                      | The content of the alert.                 |
