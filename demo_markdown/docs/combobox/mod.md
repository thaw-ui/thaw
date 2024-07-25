# Combobox

```rust demo
let selected_options = RwSignal::new(None::<String>);

view! {
    <Combobox selected_options>
        <ComboboxOption value="cat" text="Car" />
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```

### Clearable

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <Combobox selected_options clearable=true>
        <ComboboxOption value="cat" text="Car" />
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```

### Multiselect

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <Combobox selected_options>
        <ComboboxOption value="cat" text="Car" />
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```
