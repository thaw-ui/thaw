# Label

```rust demo
view! {
    <Label>"This is a label"</Label>
}
```

### Size

```rust demo
view! {
    <Flex>
        <Label size=LabelSize::Small>"Small"</Label>
        <Label>"Medium"</Label>
        <Label size=LabelSize::Large>"Large"</Label>
    </Flex>
}
```

### Weight

```rust demo
view! {
    <Flex>
        <Label>"Label"</Label>
        <Label weight=LabelWeight::Semibold>"Strong label"</Label>
    </Flex>
}
```

### Disabled

```rust demo
view! {
    <Label required=true disabled=true>"Required label"</Label>
}
```

### Required

```rust demo
view! {
    <Label required=true>"Required label"</Label>
}
```

### Label Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| size | `Signal<LabelSize>` | `LabelSize::Medium` | A label supports different sizes. |
| weight | `Signal<LabelWeight>` | `LabelWeight::Regular` | A label supports regular and semibold fontweight. |
| disabled | `Signal<bool>` | `false` | Renders the label as disabled. |
| required | `Signal<bool>` | `false` | Displays an indicator that the label is for a required field. |
| children | `Children` |  |  |
