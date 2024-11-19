# Menu

```rust demo
let toaster = ToasterInjection::expect_context();

let on_select = move |key: String| {
  leptos::logging::warn!("{}", key);
  toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastBody>
                "key"
            </ToastBody>
        </Toast>
  }, Default::default());
};

view! {
    <Space>
        <Menu on_select trigger_type=MenuTriggerType::Hover>
            <MenuTrigger slot>
                <Button>"Hover"</Button>
            </MenuTrigger>
            <MenuItem value="facebook" icon=icondata::AiFacebookOutlined>"Facebook"</MenuItem>
            <MenuItem value="twitter" disabled=true icon=icondata::AiTwitterOutlined>"Twitter"</MenuItem>
        </Menu>

        <Menu on_select>
            <MenuTrigger slot>
                <Button>"Click"</Button>
            </MenuTrigger>
            <MenuItem value="facebook" icon=icondata::AiFacebookOutlined>"Facebook"</MenuItem>
            <MenuItem value="twitter" icon=icondata::AiTwitterOutlined>"Twitter"</MenuItem>
            <MenuItem value="no_icon" disabled=true>"Mastodon"</MenuItem>
        </Menu>
    </Space>
}
```

### Placement

```rust demo
use leptos_meta::Style;

let on_select = move |value| leptos::logging::warn!("{}", value);

view! {
    <Style>
        ".demo-menu .thaw-button { width: 100% } .demo-menu .thaw-menu-trigger { display: block }"
    </Style>
    <Grid x_gap=8 y_gap=8 cols=3 class="demo-menu">
        <GridItem>
            <Menu on_select position=MenuPosition::TopStart>
                <MenuTrigger slot>
                    <Button>"Top Start"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::Top>
                <MenuTrigger slot>
                    <Button>"Top"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::TopEnd>
                <MenuTrigger slot>
                    <Button>"Top End"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::LeftStart>
                <MenuTrigger slot>
                    <Button>"Left Start"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem offset=1>
            <Menu on_select position=MenuPosition::RightStart>
                <MenuTrigger slot>
                    <Button>"Right Start"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::Left>
                <MenuTrigger slot>
                    <Button>"Left"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem offset=1>
            <Menu on_select position=MenuPosition::Right>
                <MenuTrigger slot>
                    <Button>"Right"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::LeftEnd>
                <MenuTrigger slot>
                    <Button>"Left End"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem offset=1>
            <Menu on_select position=MenuPosition::RightEnd>
                <MenuTrigger slot>
                    <Button>"Right End"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::BottomStart>
                <MenuTrigger slot>
                    <Button>"Bottom Start"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::Bottom>
                <MenuTrigger slot>
                    <Button>"Bottom"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
        <GridItem>
            <Menu on_select position=MenuPosition::BottomEnd>
                <MenuTrigger slot>
                    <Button>"Bottom End"</Button>
                </MenuTrigger>
                <MenuItem value="content">"Content"</MenuItem>
            </Menu>
        </GridItem>
    </Grid>
}
```

### Menu Props

| Name         | Type                        | Default                  | Description                                  |
| ------------ | --------------------------- | ------------------------ | -------------------------------------------- |
| class        | `MaybeProp<String>,`        | `Default::default()`     |                                              |
| on_select    | `BoxOneCallback<String>`    |                          | Called when item is selected.                |
| trigger_type | `MenuTriggerType`           | `MenuTriggerType::Click` | Action that displays the menu.               |
| position     | `MenuPosition`              | `MenuPosition::Bottom`   | Menu position.                               |
| appearance   | `MaybeProp<MenuAppearance>` | `Default::default()`     |                                              |
| menu_trigger | slot `MenuTrigger`          |                          | The element or component that triggers menu. |
| children     | `Children`                  |                          |                                              |

### MenuTriger Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### MenuItem Props

| Name     | Type                             | Default              | Description                        |
| -------- | -------------------------------- | -------------------- | ---------------------------------- |
| class    | `MaybeProp<String>`              | `Default::default()` |                                    |
| value    | `Signal<String>`                 | `Default::default()` | The value of the menu item.        |
| icon     | `MaybeProp<icondata_core::Icon>` | `None`               | The icon of the menu item.         |
| disabled | `Signal<bool>`                   | `false`              | Whether the menu item is disabled. |
| children | `Children`                       |                      |                                    |
