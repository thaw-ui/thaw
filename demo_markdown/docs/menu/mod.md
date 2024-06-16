# Menu

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Menu value default_expanded_keys=vec![String::from("area")]>
        <MenuItem key="a" label="And"/>
        <MenuItem key="o" label="Or"/>
        <MenuItem icon=icondata::AiAreaChartOutlined key="area" label="Area Chart">
            <MenuItem key="target" label="Target"/>
            <MenuItem key="above" label="Above"/>
            <MenuItem key="below" label="Below"/>
        </MenuItem>
        <MenuItem icon=icondata::AiPieChartOutlined key="pie" label="Pie Chart">
            <MenuItem key="pie-target" label="Target"/>
            <MenuItem key="pie-above" label="Above"/>
            <MenuItem key="pie-below" label="Below"/>
        </MenuItem>
        <MenuItem icon=icondata::AiGithubOutlined key="github" label="Github"/>
        <MenuItem icon=icondata::AiChromeOutlined key="chrome" label="Chrome"/>
    </Menu>
}
```

### Menu Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu element. |
| value | `Model<String>` | `Default::default()` | The selected item key of the menu. |
| default_expanded_keys | `Vec<String>` | `Default::default()` | The default expanded submenu keys. |
| children | `Children` |  | Menu's content. |

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
| children | `Option<Children>` | `None` | MenuItem's content. |
