# Select

```rust demo
let value = create_rw_signal(None::<String>);

let options = vec![
    SelectOption {
        label: String::from("RwSignal"),
        value: String::from("rw_signal"),
    },
    SelectOption {
        label: String::from("Memo"),
        value: String::from("memo"),
    },
];

view! {
    <Select value options/>
}
```

### Select Props

| Name    | Type                                | Default              | Description                               |
| ------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class   | `MaybeSignal<String>`               | `Default::default()` | Addtional classes for the select element. |
| value   | `RwSignal<Option<T>>`               | `None`               | Checked value.                            |
| options | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |
