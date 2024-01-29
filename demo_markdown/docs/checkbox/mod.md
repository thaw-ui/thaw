# Checkbox

```rust demo
let value = create_rw_signal(false);

view! {
    <Checkbox value>"Click"</Checkbox>
    <Checkbox />
}
```

### Group

```rust demo
use std::collections::HashSet;

let value = create_rw_signal(HashSet::new());

view! {
    <CheckboxGroup value>
        <CheckboxItem label="apple" key="a"/>
        <CheckboxItem label="b" key="b"/>
        <CheckboxItem label="c" key="c"/>
    </CheckboxGroup>
    <div style="margin-top: 1rem">"value: " {move || format!("{:?}", value.get())}</div>
}
```

### Checkbox Props

| Name     | Type                  | Default              | Description                                 |
| -------- | --------------------- | -------------------- | ------------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the checkbox element. |
| value    | `RwSignal<bool>`      | `false`              | Whether the checkbox is being checked.      |
| children | `Children`            |                      | Checkbox's content.                         |

### CheckboxGroup Props

| Name     | Type                        | Default              | Description                           |
| -------- | --------------------------- | -------------------- | ------------------------------------- |
| value    | `RwSignal<HashSet<String>>` | `Default::default()` | Sets the value of the checkbox group. |
| children | `Children`                  |                      | CheckboxGroup's content.              |

### CheckboxItem Props

| Name  | Type                  | Default              | Description                                             |
| ----- | --------------------- | -------------------- | ------------------------------------------------------- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the checkbox element.             |
| key   | `String`              |                      | The key of the checkbox to be used in a checkbox group. |
| label | `Option<String>`      | `None`               | Checkbox's label.                                       |
