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
        MessageOptions {closable: true, duration: std::time::Duration::from_secs(0)},
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

### MessageProvider Props

| Name      | Type                          | Default                 | Desciption                      |
| --------- | ----------------------------- | ----------------------- | ------------------------------- |
| placement | `MessagePlacement`            | `MessagePlacement::Top` | Position to place the messages. |

### MessageProvider Injection Methods

| Name   | Type                                                                           | Description              |
| ------ | ------------------------------------------------------------------------------ | ------------------------ |
| create | `fn(&self, content: String, variant: MessageVariant, options: MessageOptions)` | Use create type message. |

### MessageOptions fields

| Name     | Type              | Default                   | Description                                                     |
| -------- | ----------------- | ------------------------- | --------------------------------------------------------------- |
| duration | `Duration`        | `Duration::from_secs(3)`  | How long the message will be displayed. 0 for permanent message |
| closable | `bool`            | `false`                   | Can the message be manually closed.                             |
