# Icon

```rust demo
view! {
    <Space>
        <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCloseOutlined)/>
        <Icon icon=icondata::Icon::from(icondata::AiIcon::AiCheckOutlined)/>
    </Space>
}
```

### Icon Props

| Name   | Type                          | Default              | Desciption              |
| ------ | ----------------------------- | -------------------- | ----------------------- |
| class  | `Option<MaybeSignal<String>>` | `Default::default()` | HTML class attribute.   |
| style  | `Option<MaybeSignal<String>>` | `Default::default()` | HTML style attribute.   |
| icon   | `MaybeSignal<Icon>`           |                      | The icon to show.       |
| width  | `Option<MaybeSignal<String>>` | `1em`                | The width of the icon.  |
| height | `Option<MaybeSignal<String>>` | `1em`                | The height of the icon. |
