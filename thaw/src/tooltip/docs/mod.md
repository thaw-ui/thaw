# Tooltip

```rust demo
view! {
    <Tooltip content="Example tooltip">
        <Button>
            "Example"
        </Button>
    </Tooltip>
}
```

### Appearance: inverted

```rust demo
view! {
    <Tooltip content="Example tooltip" appearance=TooltipAppearance::Inverted>
        <Button>
            "Example"
        </Button>
    </Tooltip>
}
```

### Tooltip Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| content | `Option<Signal<String>>` | `None` | The text of the tooltip. |
| position | `TooltipPosition` | `TooltipPosition::Top` | Configure the position of the tooltip. |
| appearance | `Signal<TooltipAppearance>` | `TooltipAppearance::None` | The tooltip's visual appearance. |
| children | `T: AddAnyAttr + IntoView + Send + 'static` |  |  |
