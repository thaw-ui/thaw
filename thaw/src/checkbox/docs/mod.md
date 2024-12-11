# Checkbox

```rust demo
let checked = RwSignal::new(false);

view! {
    <Checkbox checked label="Click"/>
    <Checkbox />
}
```

### Size

```rust demo
view! {
    <Checkbox label="Medium"/>
    <Checkbox size=CheckboxSize::Large label="Large"/>
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

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| checked | `Model<bool>` | `false` | The controlled value for the checkbox. |
| value | `Option<String>` | `None` | The value of the checkbox to be used in a checkbox group. |
| label | `MaybeProp<String>` | `Default::default()` | The Checkbox's label. |
| size | `Signal<CheckboxSize>` | `CheckboxSize::Medium` | The size of the checkbox indicator. |

### CheckboxGroup Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<CheckboxGroupRule>` | `vec![]` | The rules to validate Field. |
| value | `Model<HashSet<String>>` | `Default::default()` | Sets the value of the checkbox group. |
| children | `Children` |  |  |
