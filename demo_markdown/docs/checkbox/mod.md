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

| Name     | Type                                | Default              | Description                                 |
| -------- | ----------------------------------- | -------------------- | ------------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the checkbox element. |
| value    | `Model<bool>`                       | `false`              | Whether the checkbox is being checked.      |
| children | `Children`                          |                      | Checkbox's content.                         |

### CheckboxGroup Props

| Name     | Type                     | Default              | Description                           |
| -------- | ------------------------ | -------------------- | ------------------------------------- |
| value    | `Model<HashSet<String>>` | `Default::default()` | Sets the value of the checkbox group. |
| children | `Children`               |                      | CheckboxGroup's content.              |

### CheckboxItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the checkbox element. |
| key | `String` |  | The key of the checkbox to be used in a checkbox group. |
| label | `Option<String>` | `None` | Checkbox's label. |
