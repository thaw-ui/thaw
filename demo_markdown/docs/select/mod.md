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

### Multiselect

```rust demo
let value = create_rw_signal(vec![]);

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
    <Multiselect value options/>
}
```

### Select Props

| Name    | Type                                | Default              | Description                               |
| ------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class   | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| value   | `Model<Option<T>>`                  | `None`               | Checked value.                            |
| options | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |
