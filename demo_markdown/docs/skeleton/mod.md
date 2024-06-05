# Skeleton

```rust demo
view! {
    <Skeleton>
        <SkeletonItem/>
    </Skeleton>
}
```

### Skeleton Props

| Name   | Type                          | Default | Description       |
| ------ | ----------------------------- | ------- | ----------------- |
| repeat | `MaybeSignal<u32>`            | `1`     | Repeat frequency. |
| text   | `MaybeSignal<bool>`           | `false` | Text skeleton.    |
| width  | `Option<MaybeSignal<String>>` | `None`  | Skeleton width.   |
| height | `Option<MaybeSignal<String>>` | `None`  | Skeleton height.  |
