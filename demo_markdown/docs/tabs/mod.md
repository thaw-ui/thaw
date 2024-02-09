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

### Custom tab label

```rust demo
use leptos_meta::Style;
let value = create_rw_signal(String::from("apple"));

view! {
    <Style id="demo-tab-label">
        ".p-0 { padding: 0 }"
    </Style>
    <Tabs value>
        <Tab key="apple">
            <TabLabel slot class="p-0">
                "🍎 Apple"
            </TabLabel>
            "apple"
        </Tab>
        <Tab key="pear">
            <TabLabel slot>
                "🍐 Pear"
            </TabLabel>
            "pear"
        </Tab>
    </Tabs>
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
