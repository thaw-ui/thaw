# Slider

```rust demo
let value = RwSignal::new(0.0);

view! {
    <Slider value/>
}
```

### Step

```rust demo
let value = RwSignal::new(0.0);

view! {
    <Slider step=25.0 value/>
}
```

## Slider Label

```rust demo
let value = RwSignal::new(0.0);

view! {
    <Slider value max=10.0 step=5.0>
        <SliderLabel value=0.0>
            "0"
        </SliderLabel>
        <SliderLabel value=5.0>
            "5"
        </SliderLabel>
        <SliderLabel value=10.0>
            "10"
        </SliderLabel>
    </Slider>
}
```

### Slider Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<InputRule>` | `vec![]` | The rules to validate Field. |
| value | `Model<f64>` | `0` | The current value of the controlled Slider. |
| min | `Signal<f64>` | `0` | Min value of the slider. |
| max | `Signal<f64>` | `100` | Max value of the slider. |
| step | `Signal<f64>` | `0` | The step in which value is incremented. |
| children | `Option<Children>` | `None` |  |

### SliderLabel props

| Name     | Type                | Default              | Description                          |
| -------- | ------------------- | -------------------- | ------------------------------------ |
| class    | `MaybeProp<String>` | `Default::default()` |                                      |
| value    | `Signal<f64>`       |                      | Value at which label will be placed. |
| children | `Children`          |                      |                                      |
