# Divider

```rust demo
view! {
    <Space vertical=true>
        <div style="padding: 30px 0; background-color: var(--colorNeutralBackground1);">
            <Divider />
        </div>
        <div style="padding: 30px 0; background-color: var(--colorNeutralBackground1);">
            <Divider>"Text"</Divider>
        </div>
    </Space>
}
```

### Vertical

```rust demo
view! {
    <Space vertical=true gap=SpaceGap::Large>
        <div style="height: 100px; background-color: var(--colorNeutralBackground1);">
            <Divider vertical=true/>
        </div>
        <div style="height: 100px; background-color: var(--colorNeutralBackground1);">
            <Divider vertical=true>"Text"</Divider>
        </div>
    </Space>
}
```

### Divider Props

| Name  | Type                                | Default              | Desciption                                 |
| ----- | ----------------------------------- | -------------------- | ------------------------------------------ |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the divider element. |
