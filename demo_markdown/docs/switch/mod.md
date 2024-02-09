# Switch

```rust demo
let value = create_rw_signal(false);

view! {
    <Switch value />
}
```

### Switch Props

| Name  | Type                                | Default              | Description                               |
| ----- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the switch element. |
| value | `Model<bool>`                       | `false`              | Switch's value.                           |
