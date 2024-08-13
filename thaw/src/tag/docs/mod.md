# Tag

```rust demo
view! {
    <Tag>"default"</Tag>
}
```

### Closable

```rust demo
// let message = use_message();
let success = move |_: ev::MouseEvent| {
    // message.create(
    //     "tag close".into(),
    //     MessageVariant::Success,
    //     Default::default(),
    // );
};

view! {
    <Tag closable=true on_close=success>"Default"</Tag>
}
```

### Tag Props

| Name     | Type                                     | Default              | Description                           |
| -------- | ---------------------------------------- | -------------------- | ------------------------------------- |
| class    | `MaybeProp<String>`                      | `Default::default()` |                                       |
| closable | `MaybeSignal<bool>`                      | `false`              | Whether the tag shows a close button. |
| on_close | `Option<ArcOneCallback<ev::MouseEvent>>` | `None`               | Close clicked callback.               |
| children | `Children`                               |                      |                                       |
