# Icon

```rust demo
view! {
    <Space>
        <Icon icon=icondata::AiCloseOutlined/>
        <Icon icon=icondata::AiCheckOutlined/>
    </Space>
}
```

### Icon Props

| Name     | Type                                     | Default              | Desciption                          |
| -------- | ---------------------------------------- | -------------------- | ----------------------------------- |
| class    | `MaybeProp<String>`                      | `Default::default()` | HTML class attribute.               |
| style    | `Option<Signal<String>>`                 | `Default::default()` | HTML style attribute.               |
| icon     | `Signal<icondata_core::Icon>`            |                      | The icon to show.                   |
| width    | `MaybeProp<String>`                      | `1em`                | The width of the icon.              |
| height   | `MaybeProp<String>`                      | `1em`                | The height of the icon.             |
| on_click | `Option<BoxOneCallback<ev::MouseEvent>>` | `None`               | Callback when clicking on the icon. |
