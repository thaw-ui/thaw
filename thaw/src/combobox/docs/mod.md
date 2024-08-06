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

### Grouped

```rust demo

let land = vec!["Cat".to_string(), "Dog".to_string(), "Ferret".to_string(), "Hamster".to_string()];
let water = vec!["Fish".to_string(), "Jellyfish".to_string(), "Octopus".to_string(), "Seal".to_string()];

view! {
    <Combobox>
        <ComboboxOptionGroup label="Land">
            {
                land.into_iter().map(|option| view!{
                    <ComboboxOption value={option.clone()}>
                        {option}
                    </ComboboxOption>
                }).collect_view()
            }
        </ComboboxOptionGroup>
        <OptionGroup label="Sea">
            {
                water.into_iter().map(|option| view!{
                    <ComboboxOption value={option.clone()}>
                        {option}
                    </ComboboxOption>
                }).collect_view()
            }
        </OptionGroup>
    </Combobox>
}
```
