# Space

```rust demo
view! {
    <Space>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Space>
}
```

### Vertical

```rust demo
view! {
    <Space vertical=true>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Space>
}
```

### Gap

```rust demo
view! {
    <Space gap=SpaceGap::Large>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Space>
    <Space gap=SpaceGap::WH(36, 36)>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Space>
}
```

### Space Props

| Name     | Type                  | Default              | Description                              |
| -------- | --------------------- | -------------------- | ---------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the space element. |
| vertical | `bool`                | `false`              | Whether to lay out vertically.           |
| gap      | `SpaceGap`            | `SpaceGap::Medium`   | Space's gap.                             |
| children | `Children`            |                      | Space's content.                         |
