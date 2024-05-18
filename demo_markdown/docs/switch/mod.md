# Switch

```rust demo
let value = create_rw_signal(false);

let message = use_message();
let on_change = move |value: bool| {
    message.create(
        format!("{value}"),
        MessageVariant::Success,
        Default::default(),
    );
};

view! {
    <Switch value on_change/>
}
```

### Switch Props

| Name      | Type                                | Default              | Description                                 |
| --------- | ----------------------------------- | -------------------- | ------------------------------------------- |
| class     | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the switch element.   |
| value     | `Model<bool>`                       | `false`              | Switch's value.                             |
| on_change | `Option<Callback>`                  | `None`               | Trigger when the checked state is changing. |
