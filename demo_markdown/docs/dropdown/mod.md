# Dropdown 

```rust demo
let message = use_message();

let on_select = move |key: String| {
    match key.as_str() {
        "facebook" => message.create( "Facebook".into(), MessageVariant::Success, Default::default(),),
        "twitter" =>  message.create( "Twitter".into(), MessageVariant::Warning, Default::default(),),
        _ => ()
    }
};


view! {
    <Space>
        <Dropdown on_select trigger_type=DropdownTriggerType::Hover>
            <DropdownTrigger slot>
                <Button>"Hover"</Button>
            </DropdownTrigger>
            <DropdownItem key="facebook" icon=icondata::AiFacebookOutlined label="Facebook"></DropdownItem>
            <DropdownItem key="twitter" disabled=true icon=icondata::AiTwitterOutlined label="Twitter"></DropdownItem>
        </Dropdown>

        <Dropdown on_select>
            <DropdownTrigger slot>
                <Button>"Click"</Button>
            </DropdownTrigger>
            <DropdownItem key="facebook" icon=icondata::AiFacebookOutlined label="Facebook"></DropdownItem>
            <DropdownItem key="twitter" icon=icondata::AiTwitterOutlined label="Twitter"></DropdownItem>
            <DropdownItem key="no_icon" disabled=true label="Mastodon"></DropdownItem>
        </Dropdown>
    </Space>
}
```

### Placement

```rust demo
use leptos_meta::Style;

let on_select = move |key| println!("{}", key);

view! {
    <Style>
        ".demo-dropdown .thaw-button { width: 100% } .demo-dropdown .thaw-dropdown-trigger { display: block }"
    </Style>
    <Grid x_gap=8 y_gap=8 cols=3 class="demo-dropdown">
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::TopStart>
                <DropdownTrigger slot>
                    <Button>"Top Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::Top>
                <DropdownTrigger slot>
                    <Button>"Top"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::TopEnd>
                <DropdownTrigger slot>
                    <Button>"Top End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::LeftStart>
                <DropdownTrigger slot>
                    <Button>"Left Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown on_select placement=DropdownPlacement::RightStart>
                <DropdownTrigger slot>
                    <Button>"Right Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::Left>
                <DropdownTrigger slot>
                    <Button>"Left"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown on_select placement=DropdownPlacement::Right>
                <DropdownTrigger slot>
                    <Button>"Right"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::LeftEnd>
                <DropdownTrigger slot>
                    <Button>"Left End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown on_select placement=DropdownPlacement::RightEnd>
                <DropdownTrigger slot>
                    <Button>"Right End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::BottomStart>
                <DropdownTrigger slot>
                    <Button>"Bottom Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::Bottom>
                <DropdownTrigger slot>
                    <Button>"Bottom"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown on_select placement=DropdownPlacement::BottomEnd>
                <DropdownTrigger slot>
                    <Button>"Bottom End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
    </Grid>
}
```

### Dropdown Props

| Name         | Type                                | Default                      | Description                                 |
| ------------ | ----------------------------------- | ---------------------------- | ------------------------------------------- |
| class        | `OptionalProp<MaybeSignal<String>>` | `Default::default()`         | Addtional classes for the dropdown element. |
| on_select    | `Callback<String>`                  |                              | Called when item is selected.               |
| trigger_type | `DropdownTriggerType`               | `DropdownTriggerType::Click` | Action that displays the dropdown.          |
| placement    | `DropdownPlacement`                 | `DropdownPlacement::Bottom`  | Dropdown placement.                         | 
| children     | `Children`                          |                              | The content inside dropdown.                |

### DropdownItem Props

| Name     | Type                                         | Default              | Description                                      |
| -------- | -------------------------------------------- | -------------------- | ------------------------------------------------ |
| class    | `OptionalProp<MaybeSignal<String>>`          | `Default::default()` | Addtional classes for the dropdown item element. |
| key      | `MaybeSignal<String>`                        | `Default::default()` | The key of the dropdown item.                    |
| label    | `MaybeSignal<String>`                        | `Default::default()` | The label of the dropdown item.                  |
| icon     | `OptionalMaybeSignal<icondata_core::Icon>`   | `None`               | The icon of the dropdown item.                   |
| disabled | `MaybeSignal<bool>`                          | `false`              | Whether the dropdown item is disabled.           |


### Dropdown Slots

| Name            | Default | Description                                      |
| --------------- | ------- | ------------------------------------------------ |
| DropdownTrigger | `None`  | The element or component that triggers dropdown. |

### DropdownTriger Props

| Name         | Type                                | Default                      | Description                                         |
| ------------ | ----------------------------------- | ---------------------------- | --------------------------------------------------  |
| class        | `OptionalProp<MaybeSignal<String>>` | `Default::default()`         | Addtional classes for the dropdown trigger element. |
| children     | `Children`                          |                              | The content inside dropdown trigger.                |

