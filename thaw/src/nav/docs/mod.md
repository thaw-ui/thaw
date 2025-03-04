# Nav

```rust demo
let is_multiple = RwSignal::new(true);
let is_multiple_label = Memo::new(move |_| {
    if is_multiple.get() {
        "Multiple".to_string()
    } else {
        "Single".to_string()
    }
});

view! {
    <Flex>
        <NavDrawer multiple=is_multiple>
            <NavCategory value="area">
                <NavCategoryItem slot icon=icondata::AiAreaChartOutlined>
                    "Area Chart"
                </NavCategoryItem>
                <NavSubItem value="target">
                    "Target"
                </NavSubItem>
                <NavSubItem value="above">
                    "Above"
                </NavSubItem>
                <NavSubItem value="below">
                    "Below"
                </NavSubItem>
            </NavCategory>
            <NavCategory value="pie">
                <NavCategoryItem slot icon=icondata::AiPieChartOutlined>
                    "Pie Chart"
                </NavCategoryItem>
                <NavSubItem value="pie-target">
                    "Pie Target"
                </NavSubItem>
                <NavSubItem value="pin-above">
                    "Pin Above"
                </NavSubItem>
                <NavSubItem value="pin-below">
                    "Pin Below"
                </NavSubItem>
            </NavCategory>
            <NavItem
                icon=icondata::AiGithubOutlined
                value="github"
                href="https://github.com/thaw-ui/thaw"
                attr:target="_blank"
            >
                "Github"
            </NavItem>
            <NavItem icon=icondata::AiChromeOutlined value="chrome">
                "Chrome"
            </NavItem>
        </NavDrawer>
        <div>
            <Switch checked=is_multiple label=is_multiple_label/>
        </div>
    </Flex>
}
```

### NavDrawer Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>,` | `Default::default()` |  |
| selected_value | `OptionModel<String>` | `Default::default()` | The value of the currently selected navItem. |
| selected_category_value | `OptionModel<String>` | `None` | Indicates a category that has a selected child Will show the category as selected if it is closed. |
| open_categories | `Model<Vec<String>>` | `vec![]` | Controls the open categories. |
| multiple | `Signal<bool>` | `true` | Indicates if NavDrawer supports multiple open Categories at the same time. |
| nav_drawer_header | slot `Option<NavDrawerHeader>` | `None` |  |
| nav_drawer_footer | slot `Option<NavDrawerFooter>` | `None` |  |
| children | `Children` |  |  |

### NavDrawerHeader Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### NavDrawerFooter Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### NavCategory Props

| Name              | Type                   | Default | Description |
| ----------------- | ---------------------- | ------- | ----------- |
| value             | `Signal<String>`       |         |             |
| nav_category_item | slot `NavCategoryItem` |         |             |
| children          | `Children`             |         |             |

### NavCategoryItem Props

| Name     | Type                            | Default              | Description |
| -------- | ------------------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>,`            | `Default::default()` |             |
| icon     | `MaybeProp<icondata_core::Icon` | `None`               |             |
| children | `Children`                      |                      |             |

### NavItem Props

| Name     | Type                            | Default              | Description |
| -------- | ------------------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>,`            | `Default::default()` |             |
| icon     | `MaybeProp<icondata_core::Icon` | `None`               |             |
| value    | `Signal<String>`                |                      |             |
| href     | `Option<Signal<String>>`        | `None`               |             |
| children | `Children`                      |                      |             |
