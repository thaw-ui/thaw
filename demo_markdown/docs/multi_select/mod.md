# Select Multiple

```rust demo
let values = create_rw_signal(vec![
    "rust".to_string(),
    "javascript".to_string(),
    "zig".to_string(),
    "python".to_string(),
    "cpp".to_string(),
]);

let options = vec![
    SelectOption::new("Rust", String::from("rust")).with_variant(TagVariant::Success),
    SelectOption::new("JavaScript", String::from("javascript")),
    SelectOption::new("Python", String::from("python")).with_variant(TagVariant::Warning),
    SelectOption::new("C++", String::from("cpp")).with_variant(TagVariant::Error),
    SelectOption::new("Lua", String::from("lua")),
    SelectOption::new("Zig", String::from("zig")),
];

view! {
    <MultiSelect values options />
}
```

### Select Multiple Props

| Name    | Type                                | Default              | Description                               |
| ------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class   | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| values  | `Model<Vec<T>>`                     | `vec![]`             | Checked values.                           |
| options | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |
