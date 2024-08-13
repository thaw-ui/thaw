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

| Name           | Type                | Default              | Description                              |
| -------------- | ------------------- | -------------------- | ---------------------------------------- |
| class          | `MaybeProp<String>` | `Default::default()` |                                          |
| selected_value | `Model<String>`     | `Default::default()` | The value of the currently selected tab. |
| children       | `Children`          |                      |                                          |

### Tab Props

| Name     | Type                | Default              | Description                 |
| -------- | ------------------- | -------------------- | --------------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                             |
| value    | `String`            |                      | The indentifier of the tab. |
| children | `Children`          |                      |                             |
