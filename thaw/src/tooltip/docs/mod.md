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

### Textarea Props

| Name       | Type                        | Default                   | Description                            |
| ---------- | --------------------------- | ------------------------- | -------------------------------------- |
| class      | `MaybeProp<String>`         | `Default::default()`      |                                        |
| content    | `Option<Signal<String>>`    | `None`                    | The text of the tooltip.               |
| position   | `TooltipPosition`           | `TooltipPosition::Top`    | Configure the position of the tooltip. |
| appearance | `Signal<TooltipAppearance>` | `TooltipAppearance::None` | The tooltip's visual appearance.       |
| children   | `Children`                  |                           |                                        |
