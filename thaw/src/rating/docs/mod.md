# Rating

```rust demo
let value = RwSignal::new(0.0);

view! {
    {move || value.get()}
    <Rating value />
}
```

### Step

```rust demo
let value = RwSignal::new(3.5);

view! {
    {move || value.get()}
    <Rating value step=0.5 />
}
```

### Max

```rust demo
let value = RwSignal::new(5.0);

view! {
    <Rating value max=10 />
}
```

### Color

```rust demo
view! {
    <Flex vertical=true inline=true>
        <Rating />
        <Rating color=RatingColor::Brand/>
    </Flex>
}
```

### Rating Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Model<f32>` | `0.0` | The value of the rating. |
| max | `Signal<u8>` | `5` | The max value of the rating. This controls the number of rating items displayed. Must be a whole number greater than 1. |
| step | `Signal<f32>` | `1.0` | Sets the precision to allow half-filled shapes in Rating. |
| color | `Signal<RatingColor>` | `RatingColor::Neutral` | Rating color. |
