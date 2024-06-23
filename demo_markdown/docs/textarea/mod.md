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

### Textarea Props

