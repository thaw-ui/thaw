# RatingDisplay

The value controls the number of filled stars, and is written out next to the RatingDisplay. The number of filled stars is rounded to the nearest half-star.

```rust demo
view! {
    <Flex vertical=true inline=true>
        <RatingDisplay value=1.0 />
        <RatingDisplay value=3.7 />
        <RatingDisplay value=3.9 />
        <RatingDisplay value=4.0 />
        <RatingDisplay value=5.0 />
    </Flex>
}
```

### Max

```rust demo
let value = RwSignal::new(5.0);

view! {
    <RatingDisplay value max=10 />
}
```

### Size

```rust demo
view! {
    <Flex vertical=true inline=true>
        <RatingDisplay value=3.0 size=RatingSize::Small/>
        <RatingDisplay value=3.0 size=RatingSize::Medium/>
        <RatingDisplay value=3.0 size=RatingSize::Large/>
        <RatingDisplay value=3.0 size=RatingSize::ExtraLarge/>
    </Flex>
}
```

### Color

```rust demo
view! {
    <Flex vertical=true inline=true>
        <RatingDisplay value=3.0/>
        <RatingDisplay color=RatingColor::Brand value=3.0/>
        <RatingDisplay color=RatingColor::Marigold value=3.0/>
    </Flex>
}
```

### Rating Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Signal<f32>` | `0.0` | The value of the rating. |
| max | `Signal<u8>` | `5` | The max value of the rating. This controls the number of rating items displayed. Must be a whole number greater than 1. |
| size | `Signal<RatingSize>` | `RatingSize::Medium` | Sets the size of the Rating items. |
| color | `Signal<RatingColor>` | `RatingColor::Neutral` | Rating color. |
