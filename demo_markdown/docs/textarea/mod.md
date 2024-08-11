# Input

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Textarea value placeholder="Textarea"/>
}
```

### Disabled

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Textarea value disabled=true/>
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

