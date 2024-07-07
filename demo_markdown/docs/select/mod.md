# Select

```rust demo
let value = RwSignal::new(None::<String>);

let options = vec![
    SelectOption::new("RwSignal", String::from("rw_signal")),
    SelectOption::new("Memo", String::from("memo")),
];

view! {
    <Select value options />
}
```

# Multiple Select

```rust demo
let value = RwSignal::new(vec![
    "rust".to_string(),
    "javascript".to_string(),
    "zig".to_string(),
    "python".to_string(),
    "cpp".to_string(),
]);

let options = vec![
    MultiSelectOption::new("Rust", String::from("rust")).with_variant(TagVariant::Success),
    MultiSelectOption::new("JavaScript", String::from("javascript")),
    MultiSelectOption::new("Python", String::from("python")).with_variant(TagVariant::Warning),
    MultiSelectOption::new("C++", String::from("cpp")).with_variant(TagVariant::Error),
    MultiSelectOption::new("Lua", String::from("lua")),
    MultiSelectOption::new("Zig", String::from("zig")),
];

view! {
    <MultiSelect value options />
}
```

### Select Props

| Name    | Type                                | Default              | Description                               |
| ------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class   | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| value   | `Model<Option<T>>`                  | `None`               | Checked value.                            |
| options | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |

### Multiple Select Props

| Name      | Type                                | Default              | Description                               |
| --------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class     | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| value     | `Model<Vec<T>>`                     | `vec![]`             | Checked values.                           |
| options   | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |
| clearable | `MaybeSignal<bool>`                 | `false`              | Allow the options to be cleared.          |

### Select Slots

| Name        | Default | Description   |
| ----------- | ------- | ------------- |
| SelectLabel | `None`  | Select label. |
