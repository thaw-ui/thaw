# Back Top

BackTop will find its first scrollable ascendant element and listen scroll event on it.

```rust demo remove-scrollbar
view! {
    <BackTop />
}
```

### Visibility height

```rust demo remove-scrollbar
view! {
    <BackTop bottom=100 visibility_height=280>
        <div style="width: 200px; text-align: center;">
            "Visibility Height: 280px"
        </div>
    </BackTop>
}
```

### Change position

```rust demo remove-scrollbar
view! {
    <BackTop right=40 bottom=160>
        <div style="width: 200px; text-align: center;">
            "Change Position"
        </div>
    </BackTop>
}
```

### BackTop Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| right | `MaybeSignal<i32>` | `40` | The width of BackTop from the right side of the page. |
| bottom | `MaybeSignal<i32>` | `40` | The height of BackTop from the bottom of the page. |
| visibility_height | `MaybeSignal<i32>` | `180` | BackTop's trigger scroll top. |
| children | `Option<Children>` | `None` |  |

<div style="height: 600px">
</div>
