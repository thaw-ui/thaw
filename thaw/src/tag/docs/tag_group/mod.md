# Tag Group

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

### Sizes

```rust demo
view! {
    <Space vertical=true>
        <TagGroup size=TagSize::Small>
            <Tag >"Tag 1"</Tag>
            <Tag dismissible=true>"Tag 1"</Tag>
            <InteractionTag>
                <InteractionTagPrimary>"Tag 1"</InteractionTagPrimary>
            </InteractionTag>
        </TagGroup>
        <TagGroup size=TagSize::ExtraSmall>
            <Tag >"Tag 1"</Tag>
            <Tag dismissible=true>"Tag 1"</Tag>
            <InteractionTag>
                <InteractionTagPrimary>"Tag 1"</InteractionTagPrimary>
            </InteractionTag>
        </TagGroup>
    </Space>
}
```

### TagGroup Props

| Name        | Type                             | Default              | Description                           |
| ----------- | -------------------------------- | -------------------- | ------------------------------------- |
| class       | `MaybeProp<String>`              | `Default::default()` |                                       |
| size        | `Signal<TagSize>`                | `TagSize::Medium`    | Size of the tag.                      |
| dismissible | `Signal<bool>`                   | `false`              | A Tag can be dismissible.             |
| on_dismiss  | `Option<ArcOneCallback<String>>` | `None`               | Callback for when a tag is dismissed. |
| children    | `Children`                       |                      |                                       |
