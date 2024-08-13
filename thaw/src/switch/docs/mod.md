# Switch

```rust demo
let checked = RwSignal::new(false);

view! {
    <Switch checked />
}
```

### Switch Props

| Name    | Type                | Default              | Description                                         |
| ------- | ------------------- | -------------------- | --------------------------------------------------- |
| class   | `MaybeProp<String>` | `Default::default()` |                                                     |
| checked | `Model<bool>`       | `false`              | Defines the controlled checked state of the Switch. |
| label   | `MaybeProp<String>` | `Default::default()` | The Switch's label.                                 |
