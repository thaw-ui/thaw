# Checkbox

```rust demo
let checked = RwSignal::new(false);

view! {
    <Checkbox checked label="Click"/>
    <Checkbox />
}
```

### Group

```rust demo
use std::collections::HashSet;

let value = RwSignal::new(HashSet::new());

view! {
    <CheckboxGroup value>
        <Checkbox label="apple" value="a"/>
        <Checkbox label="b" value="b"/>
        <Checkbox label="c" value="c"/>
    </CheckboxGroup>
    <div style="margin-top: 1rem">"value: " {move || format!("{:?}", value.get())}</div>
}
```

### Checkbox Props

| Name    | Type                | Default              | Description                                               |
| ------- | ------------------- | -------------------- | --------------------------------------------------------- |
| class   | `MaybeProp<String>` | `Default::default()` |                                                           |
| checked | `Model<bool>`       | `false`              | The controlled value for the checkbox.                    |
| value   | `Option<String>`    | `None`               | The value of the checkbox to be used in a checkbox group. |
| label   | `MaybeProp<String>` | `Default::default()` | The Checkbox's label.                                     |

### CheckboxGroup Props

| Name     | Type                     | Default              | Description                           |
| -------- | ------------------------ | -------------------- | ------------------------------------- |
| class    | `MaybeProp<String>`      | `Default::default()` |                                       |
| value    | `Model<HashSet<String>>` | `Default::default()` | Sets the value of the checkbox group. |
| children | `Children`               |                      |                                       |
