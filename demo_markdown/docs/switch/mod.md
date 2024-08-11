# Switch

```rust demo
let checked = RwSignal::new(false);

view! {
    <Switch checked />
}
```

### Switch Props

| Name  | Type                                | Default              | Description                               |
| ----- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the switch element. |
| value | `Model<bool>`                       | `false`              | Switch's value.                           |
