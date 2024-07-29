# Menu

```rust demo


let on_select = |key: String| {
  leptos::logging::warn!("{}", key);
  let toaster = ToasterInjection::expect_context();
  toaster.dispatch_toast(view! {
        <Toast>
            <ToastBody>
                "key"
            </ToastBody>
        </Toast>
  }.into_any(), Default::default());
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

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu element. |
| on_select | `Callback<String>` |  | Called when item is selected. |
| trigger_type | `MenuTriggerType` | `MenuTriggerType::Click` | Action that displays the menu. |
| position | `MenuPosition` | `MenuPosition::Bottom` | Menu position. |
| children | `Children` |  | The content inside menu. |

### MenuItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu item element. |
| value | `MaybeSignal<String>` | `Default::default()` | The value of the menu item. |
| label | `MaybeSignal<String>` | `Default::default()` | The label of the menu item. |
| icon | `OptionalMaybeSignal<icondata_core::Icon>` | `None` | The icon of the menu item. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the menu item is disabled. |

### Menu Slots

| Name        | Default | Description                                  |
| ----------- | ------- | -------------------------------------------- |
| MenuTrigger | `None`  | The element or component that triggers menu. |

### MenuTriger Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the menu trigger element. |
| children | `Children` |  | The content inside menu trigger. |
