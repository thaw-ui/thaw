# Tabs

```rust demo
let value = create_rw_signal(String::from("apple"));

view! {
    <Tabs value>
        <Tab key="apple" label="Apple">
            "apple"
        </Tab>
        <Tab key="pear" label="Pear">
            "pear"
        </Tab>
    </Tabs>
}
```

### Tabs Props

| Name     | Type                  | Default               | Description                             |
| -------- | --------------------- | --------------------- | --------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()`  | Addtional classes for the tabs element. |
| value    | `RwSignal<String>`    | `TagVariant::Default` | Tabs value.                             |
| children | `Children`            |                       | Tabs content.                           |

### Tab Props

| Name     | Type                  | Default              | Description                            |
| -------- | --------------------- | -------------------- | -------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the tab element. |
| key      | `String`              |                      | The indentifier of the tab.            |
| label    | `String`              |                      | The label of the tab.                  |
| children | `Children`            |                      | Tabs content.                          |
