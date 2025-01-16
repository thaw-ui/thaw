# RangeSlider

```rust demo
let value = RwSignal::new((50.0, 70.0));

view! {
    <div>
        {move || format!("{:#?}", value.get())}
    </div>
    <RangeSlider value attr:style="width: 400px"/>
}
```

### Step

```rust demo
let value = RwSignal::new((50.0, 70.0));

view! {
    <RangeSlider step=25.0 value attr:style="width: 400px"/>
}
```

### SliderLabel

### RangeSlider Props

| Name  | Type                | Default              | Description                                 |
| ----- | ------------------- | -------------------- | ------------------------------------------- |
| class | `MaybeProp<String>` | `Default::default()` |                                             |
| value | `Model<(f64, f64)>` | `(0.0, 0.0)`         | The current value of the controlled Slider. |
| min   | `Signal<f64>`       | `0`                  | Min value of the slider.                    |
| max   | `Signal<f64>`       | `100`                | Max value of the slider.                    |
| step  | `Signal<f64>`       | `0`                  | The step in which value is incremented.     |
