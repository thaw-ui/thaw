# Switch

```rust demo
let value = create_rw_signal(false);

view! {
    <Switch value />
}
```

### Switch Props

| Name  | Type                  | Default              | Description                               |
| ----- | --------------------- | -------------------- | ----------------------------------------- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the switch element. |
| value | `RwSignal<bool>`      | `Default::default()` | Switch's value.                           |
