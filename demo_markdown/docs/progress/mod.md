# Progress

```rust demo
let percentage = create_rw_signal(0.0f32);

view! {
    <Space vertical=true>
        <Progress percentage show_indicator=false/>
        <Progress percentage/>
        <Progress percentage indicator_placement=ProgressIndicatorPlacement::Inside/>
        <Progress percentage color=ProgressColor::Success/>
        <Progress percentage color=ProgressColor::Warning/>
        <Progress percentage color=ProgressColor::Error/>
        <Space>
            <Button on_click=move |_| percentage.update(|v| *v -= 10.0)>"-10%"</Button>
            <Button on_click=move |_| percentage.update(|v| *v += 10.0)>"+10%"</Button>
        </Space>
    </Space>
}
```

### Circle

```rust demo
let percentage = create_rw_signal(0.0f32);

view! {
    <Space>
        <ProgressCircle percentage/>
        <ProgressCircle percentage color=ProgressColor::Success/>
        <ProgressCircle percentage color=ProgressColor::Warning/>
        <ProgressCircle percentage color=ProgressColor::Error/>
    </Space>
    <Space>
        <Button on_click=move |_| percentage.update(|v| *v -= 10.0)>"-10%"</Button>
        <Button on_click=move |_| percentage.update(|v| *v += 10.0)>"+10%"</Button>
    </Space>
}
```

### Progress Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| percentage | `MaybeSignal<f32>` | `Default::default()` | Percentage value. |
| color | `MaybeSignal<ProgressColor>` | `ProgressColor::Primary` | Progress color. |
| show_indicator | `MaybeSignal<bool>` | `true` | Whether to display indicators. |
| indicator_placement | `MaybeSignal<ProgressIndicatorPlacement>` | `ProgressIndicatorPlacement::Outside` | Indicator placement. |

### ProgressCircle Props

| Name       | Type                         | Default                  | Description               |
| ---------- | ---------------------------- | ------------------------ | ------------------------- |
| percentage | `MaybeSignal<f32>`           | `Default::default()`     | Percentage value.         |
| color      | `MaybeSignal<ProgressColor>` | `ProgressColor::Primary` | ProgressCircle color.     |
| children   | `Option<Children>`           | `None`                   | ProgressCircle's content. |
