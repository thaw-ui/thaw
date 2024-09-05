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

### Tag Group

```rust demo
view! {
    <Space vertical=true>
        <TagGroup attr:role="list">
            <Tag attr:role="listitem">"Tag 1"</Tag>
            <Tag attr:role="listitem">"Tag 2"</Tag>
            <Tag attr:role="listitem">"Tag 3"</Tag>
        </TagGroup>
        <TagGroup>
            <InteractionTag>
                <InteractionTagPrimary>"Tag 1"</InteractionTagPrimary>
            </InteractionTag>
            <InteractionTag>
                <InteractionTagPrimary>"Tag 2"</InteractionTagPrimary>
            </InteractionTag>
            <InteractionTag>
                <InteractionTagPrimary>"Tag 3"</InteractionTagPrimary>
            </InteractionTag>
        </TagGroup>
    </Space>
}

```

### Tag Props

| Name     | Type                                     | Default              | Description                           |
| -------- | ---------------------------------------- | -------------------- | ------------------------------------- |
| class    | `MaybeProp<String>`                      | `Default::default()` |                                       |
| closable | `MaybeSignal<bool>`                      | `false`              | Whether the tag shows a close button. |
| on_close | `Option<ArcOneCallback<ev::MouseEvent>>` | `None`               | Close clicked callback.               |
| children | `Children`                               |                      |                                       |

### InteractionTag Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### InteractionTagPrimary Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### TagGroup Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
