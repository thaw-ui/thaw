# RangeSlider

```rust demo
let value = RwSignal::new((50.0, 70.0));

view! {
    <div>
        {move || format!("{:#?}", value.get())}
    </div>
    <RangeSlider value style="width: 400px"/>
}
```

### Step

```rust demo
let value = RwSignal::new((50.0, 70.0));

view! {
    <Flex vertical=true inline=true>
        <RangeSlider step=25.0 value style="width: 400px"/>
        <RangeSlider step=25.0 value style="width: 400px" show_stops=false/>
    </Flex>
}
```

### Vertical

```rust demo
let value = RwSignal::new((6.0, 8.0));

view! {
    <RangeSlider value vertical=true step=2.0 min=0.0 max=10.0 />
}
```

### SliderLabel

```rust demo
let value = RwSignal::new((0.0, 1.0));

view! {
    <RangeSlider value max=10.0 step=5.0  style="width: 400px" >
        <SliderLabel value=0.0>
            "0"
        </SliderLabel>
        <SliderLabel value=5.0>
            "5"
        </SliderLabel>
        <SliderLabel value=10.0>
            "10"
        </SliderLabel>
    </RangeSlider>
}
```

### RangeSlider Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| style | `MaybeProp<String>` | `Default::default()` |  |
| value | `Model<(f64, f64)>` | `(0.0, 0.0)` | The current value of the controlled Slider. |
| min | `Signal<f64>` | `0` | Min value of the slider. |
| max | `Signal<f64>` | `100` | Max value of the slider. |
| step | `Signal<f64>` | `0` | The step in which value is incremented. |
| show_stops | `Signal<bool>` | `true` | Whether to display breakpoints. |
| vertical | `Signal<bool>` | `false` | Render the Slider in a vertical orientation, smallest value on the bottom. |
