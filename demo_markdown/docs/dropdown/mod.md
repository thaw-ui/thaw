# Dropdown 

```rust demo
let value = create_rw_signal(None::<String>);
let message = use_message();
let facebook = move |_| {
    message.create(
        "Facebook".into(),
        MessageVariant::Success,
        Default::default(),
    );
};
let twitter = move |_| {
    message.create(
        "Twitter".into(),
        MessageVariant::Warning,
        Default::default(),
    );
};
view! {
    <Space>
        <Dropdown>
            <DropdownTrigger slot>
                <Button>"Click"</Button>
            </DropdownTrigger>
            <DropdownItem on_click=facebook icon=icondata::AiFacebookOutlined label="Facebook"></DropdownItem>
            <DropdownItem on_click=twitter disabled=true icon=icondata::AiTwitterOutlined label="Twitter"></DropdownItem>
        </Dropdown>

        <Dropdown trigger_type=DropdownTriggerType::Hover>
            <DropdownTrigger slot>
                <Button>"Hover"</Button>
            </DropdownTrigger>
            <DropdownItem on_click=facebook icon=icondata::AiFacebookOutlined label="Facebook"></DropdownItem>
            <DropdownItem on_click=twitter disabled=true icon=icondata::AiTwitterOutlined label="Twitter"></DropdownItem>
        </Dropdown>
    </Space>
}
```

### Placement

```rust demo
use leptos_meta::Style;

view! {
    <Style>
        ".demo-dropdown .thaw-button { width: 100% } .demo-dropdown .thaw-dropdown-trigger { display: block }"
    </Style>
    <Grid x_gap=8 y_gap=8 cols=3 class="demo-dropdown">
        <GridItem>
            <Dropdown placement=DropdownPlacement::TopStart>
                <DropdownTrigger slot>
                    <Button>"Top Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::Top>
                <DropdownTrigger slot>
                    <Button>"Top"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::TopEnd>
                <DropdownTrigger slot>
                    <Button>"Top End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::LeftStart>
                <DropdownTrigger slot>
                    <Button>"Left Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown placement=DropdownPlacement::RightStart>
                <DropdownTrigger slot>
                    <Button>"Right Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::Left>
                <DropdownTrigger slot>
                    <Button>"Left"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown placement=DropdownPlacement::Right>
                <DropdownTrigger slot>
                    <Button>"Right"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::LeftEnd>
                <DropdownTrigger slot>
                    <Button>"Left End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem offset=1>
            <Dropdown placement=DropdownPlacement::RightEnd>
                <DropdownTrigger slot>
                    <Button>"Right End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::BottomStart>
                <DropdownTrigger slot>
                    <Button>"Bottom Start"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::Bottom>
                <DropdownTrigger slot>
                    <Button>"Bottom"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
        <GridItem>
            <Dropdown placement=DropdownPlacement::BottomEnd>
                <DropdownTrigger slot>
                    <Button>"Bottom End"</Button>
                </DropdownTrigger>
                "Content"
            </Dropdown>
        </GridItem>
    </Grid>
}
```

### Select Props

| Name    | Type                                | Default              | Description                               |
| ------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class   | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| value   | `Model<Option<T>>`                  | `None`               | Checked value.                            |
| options | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |

### Multiple Select Props

| Name      | Type                                | Default              | Description                               |
| --------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class     | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the select element. |
| value     | `Model<Vec<T>>`                     | `vec![]`             | Checked values.                           |
| options   | `MaybeSignal<Vec<SelectOption<T>>>` | `vec![]`             | Options that can be selected.             |
| clearable | `MaybeSignal<bool>`                 | `false`              | Allow the options to be cleared.          |

### Select Slots

| Name        | Default | Description   |
| ----------- | ------- | ------------- |
| SelectLabel | `None`  | Select label. |
