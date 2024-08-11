# ProgressBar

```rust demo
let value = RwSignal::new(0.0);

view! {
    <Space vertical=true>
        <ProgressBar value/>
        <ProgressBar value color=ProgressBarColor::Success/>
        <ProgressBar value color=ProgressBarColor::Warning/>
        <ProgressBar value color=ProgressBarColor::Error/>
        <Space>
            <Button on_click=move |_| value.update(|v| *v -= 0.1)>"-10%"</Button>
            <Button on_click=move |_| value.update(|v| *v += 0.1)>"+10%"</Button>
        </Space>
    </Space>
}
```

### Circle

```rust demo
let value = RwSignal::new(0.0);

view! {
    <Space>
        <ProgressCircle value/>
        <ProgressCircle value color=ProgressCircleColor::Success/>
        <ProgressCircle value color=ProgressCircleColor::Warning/>
        <ProgressCircle value color=ProgressCircleColor::Error/>
    </Space>
    <Space>
        <Button on_click=move |_| value.update(|v| *v -= 10.0)>"-10%"</Button>
        <Button on_click=move |_| value.update(|v| *v += 10.0)>"+10%"</Button>
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

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the progress element. |
| percentage | `MaybeSignal<f32>` | `Default::default()` | Percentage value. |
| color | `MaybeSignal<ProgressColor>` | `ProgressColor::Primary` | ProgressCircle color. |
| size | `MaybeSignal<Stringr>` | `120px` | ProgressCircle size. |
| children | `Option<Children>` | `None` | ProgressCircle's content. |
