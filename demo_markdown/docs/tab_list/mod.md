# Tabs

```rust demo
let selected_value = RwSignal::new(String::new());

view! {
    <TabList selected_value>
        <Tab value="apple">
            "Apple"
        </Tab>
        <Tab value="pear">
            "Pear"
        </Tab>
        <Tab value="item1">
            "Item 1"
        </Tab>
        <Tab value="item2">
            "Item 2"
        </Tab>
    </TabList>
}
```

### Tabs Props

| Name     | Type                                | Default              | Description                             |
| -------- | ----------------------------------- | -------------------- | --------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the tabs element. |
| value    | `Model<String>`                     | `Default::default()` | Tabs value.                             |
| children | `Children`                          |                      | Tabs content.                           |

### Tab Props

| Name     | Type                                | Default              | Description                            |
| -------- | ----------------------------------- | -------------------- | -------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the tab element. |
| key      | `String`                            |                      | The indentifier of the tab.            |
| label    | `String`                            | `Default::default()` | The label of the tab.                  |
| children | `Children`                          |                      | Tabs content.                          |

### Tab Slots

| Name     | Default | Description    |
| -------- | ------- | -------------- |
| TabLabel | `None`  | label content. |
