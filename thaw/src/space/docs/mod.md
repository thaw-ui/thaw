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

### Align

```rust demo
view! {
    <Space vertical=true>
        <Space align=SpaceAlign::Start>
            <Button attr:style="height: 60px">"Start"</Button>
            <Button attr:style="height: 40px">"Start"</Button>
            <Button>"Start"</Button>
        </Space>
        <Space align=SpaceAlign::Center>
            <Button attr:style="height: 60px">"Center"</Button>
            <Button attr:style="height: 40px">"Center"</Button>
            <Button>"Center"</Button>
        </Space>
        <Space align=SpaceAlign::End>
            <Button attr:style="height: 60px">"End"</Button>
            <Button attr:style="height: 40px">"End"</Button>
            <Button>"End"</Button>
        </Space>
    </Space>
}
```

### Justify

```rust demo
view! {
    <Space vertical=true>
        <Space justify=SpaceJustify::SpaceAround>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
        </Space>
        <Space justify=SpaceJustify::Center>
            <Button>"Center"</Button>
            <Button>"Center"</Button>
            <Button>"Center"</Button>
        </Space>
        <Space justify=SpaceJustify::End>
            <Button>"End"</Button>
            <Button>"End"</Button>
            <Button>"End"</Button>
        </Space>
    </Space>
}
```

### Space Props

| Name     | Type                      | Default              | Description                    |
| -------- | ------------------------- | -------------------- | ------------------------------ |
| class    | `MaybeProp<String>`       | `Default::default()` |                                |
| vertical | `bool`                    | `false`              | Whether to lay out vertically. |
| gap      | `SpaceGap`                | `SpaceGap::Medium`   | Space's gap.                   |
| align    | `MaybeProp<SpaceAlign>`   | `None`               | Vertical arrangement.          |
| justify  | `MaybeProp<SpaceJustify>` | `None`               | Horizontal arrangement.        |
| children | `ChildrenFragment`        |                      |                                |
