# Menu

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Menu value>
        <MenuItem key="a" label="And"/>
        <MenuItem key="o" label="Or"/>
        <MenuItem icon=icondata::AiAreaChartOutlined key="area" label="Area Chart"/>
        <MenuItem icon=icondata::AiPieChartOutlined key="pie" label="Pie Chart"/>
        <MenuItem icon=icondata::AiGithubOutlined key="github" label="Github"/>
        <MenuItem icon=icondata::AiChromeOutlined key="chrome" label="Chrome"/>
    </Menu>
}
```

### Menu Props

| Name     | Type                                | Default              | Description                             |
| -------- | ----------------------------------- | -------------------- | --------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu element. |
| value    | `Model<String>`                     | `Default::default()` | The selected item key of the menu.      |
| children | `Children`                          |                      | Menu's content.                         |

### MenuGroup Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu group element. |
| label | `String` |  | The label of the menu group. |
| children | `Children` |  | MenuGroup's content. |

### MenuItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu item element. |
| label | `MaybeSignal<String>` | `Default::default()` | The label of the menu item. |
| key | `MaybeSignal<String>` | `Default::default()` | The indentifier of the menu item. |
| icon | `OptionalMaybeSignal<icondata_core::Icon>` | `None` | The icon of the menu item. |
