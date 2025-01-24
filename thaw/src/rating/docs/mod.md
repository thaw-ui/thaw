# Rating

```rust demo
let value = RwSignal::new(0.0);

view! {
    {move || value.get()}
    <Rating value />
}
```