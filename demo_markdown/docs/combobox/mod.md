# Combobox

```rust demo
let value = RwSignal::new(vec![]);

view! {
    <Combobox value>
        <ComboboxOption key="cat">
            "Cat"
        </ComboboxOption>
        <ComboboxOption key="dog">
            "Dog"
        </ComboboxOption>
    </Combobox>
}
```

### Clearable

```rust demo
let value = RwSignal::new(vec![]);

view! {
    <Combobox value multiselect=true clearable=true>
        <ComboboxOption key="cat">
            "Cat"
        </ComboboxOption>
        <ComboboxOption key="dog">
            "Dog"
        </ComboboxOption>
    </Combobox>
}
```

### Multiselect

```rust demo
let value = RwSignal::new(vec![]);

view! {
    <Combobox value multiselect=true>
        <ComboboxOption key="cat">
            "Cat"
        </ComboboxOption>
        <ComboboxOption key="dog">
            "Dog"
        </ComboboxOption>
    </Combobox>
}
```
