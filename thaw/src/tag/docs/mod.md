# Tag

```rust demo
view! {
    <Space>
        <Tag>"default"</Tag>
        <InteractionTag>
            <InteractionTagPrimary>
                "Interaction Tag"
            </InteractionTagPrimary>
        </InteractionTag>
    </Space>
}
```

### Size

```rust demo
view! {
    <Space vertical=true>
        <Space>
            <Tag >"Medium"</Tag>
            <Tag size=TagSize::Small>"Small"</Tag>
            <Tag size=TagSize::ExtraSmall>"Extra small"</Tag>
        </Space>
        <Space>
            <Tag closable=true>"Medium"</Tag>
            <Tag closable=true size=TagSize::Small>"Small"</Tag>
            <Tag closable=true size=TagSize::ExtraSmall>"Extra small"</Tag>
        </Space>
    </Space>
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
| size     | `Option<MaybeSignal<TagSize>>`           | `None`               | Size of the tag.                      |
| closable | `MaybeSignal<bool>`                      | `false`              | Whether the tag shows a close button. |
| on_close | `Option<ArcOneCallback<ev::MouseEvent>>` | `None`               | Close clicked callback.               |
| children | `Children`                               |                      |                                       |

### InteractionTag Props

| Name     | Type                           | Default              | Description      |
| -------- | ------------------------------ | -------------------- | ---------------- |
| class    | `MaybeProp<String>`            | `Default::default()` |                  |
| size     | `Option<MaybeSignal<TagSize>>` | `None`               | Size of the tag. |
| children | `Children`                     |                      |                  |

### InteractionTagPrimary Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
