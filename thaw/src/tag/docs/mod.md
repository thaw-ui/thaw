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
            <Tag dismissible=true>"Medium"</Tag>
            <Tag dismissible=true size=TagSize::Small>"Small"</Tag>
            <Tag dismissible=true size=TagSize::ExtraSmall>"Extra small"</Tag>
        </Space>
    </Space>
}
```

### Dismiss

```rust demo
let toaster = ToasterInjection::expect_context();

let on_dismiss = move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Tag"</ToastTitle>
            <ToastBody>
                "Tag dismiss"
            </ToastBody>
        </Toast>
     }, Default::default());
};

view! {
    <Tag dismissible=true on_dismiss=on_dismiss>"Default"</Tag>
}
```

### Tag Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| size | `Option<Signal<TagSize>>` | `None` | Size of the tag. |
| dismissible | `Signal<bool>` | `false` | A Tag can be dismissible. |
| on_dismiss | `Option<ArcOneCallback<ev::MouseEvent>>` | `None` | Callback for when a tag is dismissed. |
| value | `Option<String>` | `None` | Unique value identifying the tag within a TagGroup. |
| children | `Children` |  |  |

### InteractionTag Props

| Name     | Type                      | Default              | Description      |
| -------- | ------------------------- | -------------------- | ---------------- |
| class    | `MaybeProp<String>`       | `Default::default()` |                  |
| size     | `Option<Signal<TagSize>>` | `None`               | Size of the tag. |
| children | `Children`                |                      |                  |

### InteractionTagPrimary Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
