# Slider

```rust demo
let value = create_rw_signal(0.0);

view! {
    <Slider value/>
}
```

### Step

```rust demo
let value = create_rw_signal(0.0);

view! {
    <Slider step=10.0 value/>
}
```

## Slider Label

```rust demo
let value = create_rw_signal(0.0);

view! {
    <Slider value max=10.0 step=1.0>
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

| Name     | Type                                | Default              | Description                               |
| -------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the slider element. |
| value    | `MaybeSignal<f64>`                  | `Default::default()` | Value of the slider.                      |
| max      | `MaybeSignal<f64>`                  | `100`                | Max value of the slider.                  |
| step     | `MaybeSignal<f64>`                  | `Default::default()` | The step in which value is incremented.   |
| children | `Option<Children>`                  | `None`               | Slider's content.                         |

### SliderLabel props

| Name     | Type               | Default | Description                          |
| -------- | ------------------ | ------- | ------------------------------------ |
| value    | `MaybeSignal<f64>` |         | Value at which label will be placed. |
| children | `Children`         |         | Content of the lable.                |
