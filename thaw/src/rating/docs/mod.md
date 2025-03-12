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

### Size

```rust demo
view! {
    <Flex vertical=true inline=true>
        <Rating value=3.0 size=RatingSize::Small/>
        <Rating value=3.0 size=RatingSize::Medium/>
        <Rating value=3.0 size=RatingSize::Large/>
        <Rating value=3.0 size=RatingSize::ExtraLarge/>
    </Flex>
}
```

### Color

```rust demo
view! {
    <Flex vertical=true inline=true>
        <Rating value=3.0/>
        <Rating color=RatingColor::Brand value=3.0/>
        <Rating color=RatingColor::Marigold value=3.0/>
    </Flex>
}
```

### Rating Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | Name for the Radio inputs. If not provided, one will be automatically generated. |
| rules | `Vec<RatingRule>` | `vec![]` | The rules to validate Field. |
| value | `OptionModel<f32>` | `None` | The value of the rating. |
| max | `Signal<u8>` | `5` | The max value of the rating. This controls the number of rating items displayed. Must be a whole number greater than 1. |
| step | `Signal<f32>` | `1.0` | Sets the precision to allow half-filled shapes in Rating. |
| size | `Signal<RatingSize>` | `RatingSize::ExtraLarge` | Sets the size of the Rating items. |
| color | `Signal<RatingColor>` | `RatingColor::Neutral` | Rating color. |
