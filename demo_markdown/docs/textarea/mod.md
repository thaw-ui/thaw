# Input

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Space vertical=true>
        <Textarea value placeholder="Textarea"/>
    </Space>
}
```

### Disabled

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Space vertical=true>
        <Textarea value disabled=true/>
    </Space>
}
```

### Resize

```rust demo
view! {
    <Space vertical=true>
        <Textarea placeholder=r#"Textarea with resize set to "none""#/>
        <Textarea placeholder=r#"Textarea with resize set to "vertical""# resize=TextareaResize::Vertical/>
        <Textarea placeholder=r#"Textarea with resize set to "horizontal""# resize=TextareaResize::Horizontal/>
        <Textarea placeholder=r#"Textarea with resize set to "both""# resize=TextareaResize::Both/>
    </Space>
}
```

### Textarea Props

