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

### ProgressBar Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Signal<f64>` | `Default::default()` | A decimal number between 0 and 1 (or between 0 and max if given), which specifies how much of the task has been completed. |
| max | `Signal<f64>` | `1.0` | The maximum value, which indicates the task is complete. The ProgressBar bar will be full when value equals max. |
| color | `Signal<ProgressColor>` | `ProgressColor::Brand` | ProgressBar color. |

### ProgressCircle Props

| Name     | Type                          | Default                      | Description           |
| -------- | ----------------------------- | ---------------------------- | --------------------- |
| class    | `MaybeProp<String>`           | `Default::default()`         |                       |
| value    | `Signal<f64>`                 | `Default::default()`         | Percentage value.     |
| color    | `Signal<ProgressCircleColor>` | `ProgressCircleColor::Brand` | ProgressCircle color. |
| size     | `Signal<String>`              | `120px`                      | ProgressCircle size.  |
| children | `Option<Children>`            | `None`                       |                       |
