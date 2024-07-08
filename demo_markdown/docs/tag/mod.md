# Tag

```rust demo
view! {
    <Space>
        <Tag>"default"</Tag>
        <Tag variant=TagVariant::Success>"success"</Tag>
        <Tag variant=TagVariant::Warning>"warning"</Tag>
        <Tag variant=TagVariant::Error>"error"</Tag>
    </Space>
}
```

### Closable

```rust demo
use send_wrapper::SendWrapper;
// let message = use_message();
let success = move |_: SendWrapper<ev::MouseEvent>| {
    // message.create(
    //     "tag close".into(),
    //     MessageVariant::Success,
    //     Default::default(),
    // );
};

view! {
    <Space>
        <Tag closable=true on_close=success>"Default"</Tag>
        <Tag closable=true on_close=success variant=TagVariant::Success>"Success"</Tag>
        <Tag closable=true on_close=success variant=TagVariant::Warning>"Warning"</Tag>
        <Tag closable=true on_close=success variant=TagVariant::Error>"Error"</Tag>
    </Space>
}
```

### Tag Props

| Name     | Type                                | Default               | Description                            |
| -------- | ----------------------------------- | --------------------- | -------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()`  | Addtional classes for the tag element. |
| variant  | `MaybeSignal<TagVariant>`           | `TagVariant::Default` | Tag's variant.                         |
| closable | `MaybeSignal<bool>`                 | `false`               | Whether the tag shows a close button.  |
| on_close | `Option<Callback<ev::MouseEvent>>`  | `None`                | Close clicked callback.                |
| children | `Children`                          |                       | Tag's content.                         |
