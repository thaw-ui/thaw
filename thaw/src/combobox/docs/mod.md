# Combobox

```rust demo
let selected_options = RwSignal::new(None::<String>);

view! {
    <Combobox selected_options placeholder="Select an animal">
        <ComboboxOption value="cat" text="Cat" disabled=true/>
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```

### Clearable

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <Combobox selected_options clearable=true>
        <ComboboxOption value="cat" text="Cat" disabled=true/>
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```

### Multiselect

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <Combobox selected_options>
        <ComboboxOption value="cat" text="Cat" disabled=true/>
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
                    <ComboboxOption text={option} />
                }).collect_view()
            }
        </ComboboxOptionGroup>
        <ComboboxOptionGroup label="Sea">
            {
                water.into_iter().map(|option| view!{
                    <ComboboxOption text={option} />
                }).collect_view()
            }
        </ComboboxOptionGroup>
    </Combobox>
}
```

### Disabled

```rust demo
view! {
    <Combobox disabled=true>
        <ComboboxOption value="cat" text="Car" />
        <ComboboxOption value="dog" text="Dog" />
    </Combobox>
}
```

### Combobox Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Model<String>` | `Default::default()` |  |
| selected_options | `VecModel<String>` | `Default::default()` | Selected option. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| placeholder | `MaybeProp<String>` | `Default::default()` | Placeholder text for the input. |
| clearable | `bool` | `false` | If set, the combobox will show an icon to clear the current value. |
| children | `Children` |  |  |

### ComboboxOption Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| disabled | `MaybeSignal<bool>` | `false` | Sets an option to the disabled state. Disabled options cannot be selected, but are still keyboard navigable. |
| value | `Option<String>` | `None` | Defines a unique identifier for the option. Defaults to `text` if not provided. |
| text | `String` |  | An optional override the string value of the Option's display text, defaulting to the Option's child content. |
| children | `Children` |  |  |

### ComboboxOptionGroup Props

| Name     | Type                | Default              | Desciption          |
| -------- | ------------------- | -------------------- | ------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                     |
| label    | `String`            |                      | Label of the group. |
| children | `Children`          |                      |                     |
