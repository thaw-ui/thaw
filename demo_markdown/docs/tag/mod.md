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

### Tag Props

| Name     | Type                                | Default               | Description                            |
| -------- | ----------------------------------- | --------------------- | -------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()`  | Addtional classes for the tag element. |
| variant  | `MaybeSignal<TagVariant>`           | `TagVariant::Default` | Tag's variant.                         |
| children | `Children`                          |                       | Tag's content.                         |
