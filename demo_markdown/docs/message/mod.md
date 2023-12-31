# Message

<Alert variant=AlertVariant::Warning title="Prerequisite">
    "If you want to use message, you need to wrap the component where you call related methods inside MessageProvider and use use_message to get the API."
</Alert>

```rust demo
let message = use_message();
let success = move |_| {
    message.create(
        "Success".into(),
        MessageVariant::Success,
        Default::default(),
    );
};
let warning = move |_| {
    message.create(
        "Warning".into(),
        MessageVariant::Warning,
        Default::default(),
    );
};
let error = move |_| {
    message.create("Error".into(), MessageVariant::Error, Default::default());
};

view! {
    <Space>
        <Button on_click=success>"Success"</Button>
        <Button on_click=warning>"Warning"</Button>
        <Button on_click=error>"Error"</Button>
    </Space>
}
```

### MessageProvider Injection Methods

| Name   | Type                                                                           | Description              |
| ------ | ------------------------------------------------------------------------------ | ------------------------ |
| create | `fn(&self, content: String, variant: MessageVariant, options: MessageOptions)` | Use create type message. |
