# Popover

```rust demo
view! {
    <Space>
        <Popover>
            <PopoverTrigger slot>
                <Button>"Hover"</Button>
            </PopoverTrigger>
            "Content"
        </Popover>
        <Popover trigger_type=PopoverTriggerType::Click>
            <PopoverTrigger slot>
                <Button>"Click"</Button>
            </PopoverTrigger>
            "Content"
        </Popover>
    </Space>
}
```

### Placement

```rust demo
use leptos_meta::Style;

view! {
    <Style>
        ".demo-popover .thaw-button { width: 100% } .demo-popover .thaw-popover-trigger { display: block }"
    </Style>
    <Grid x_gap=8 y_gap=8 cols=3 class="demo-popover">
        <GridItem>
            <Popover placement=PopoverPlacement::TopStart>
                <PopoverTrigger slot>
                    <Button>"Top Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::Top>
                <PopoverTrigger slot>
                    <Button>"Top"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::TopEnd>
                <PopoverTrigger slot>
                    <Button>"Top End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::LeftStart>
                <PopoverTrigger slot>
                    <Button>"Left Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover placement=PopoverPlacement::RightStart>
                <PopoverTrigger slot>
                    <Button>"Right Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::Left>
                <PopoverTrigger slot>
                    <Button>"Left"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover placement=PopoverPlacement::Right>
                <PopoverTrigger slot>
                    <Button>"Right"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::LeftEnd>
                <PopoverTrigger slot>
                    <Button>"Left End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover placement=PopoverPlacement::RightEnd>
                <PopoverTrigger slot>
                    <Button>"Right End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::BottomStart>
                <PopoverTrigger slot>
                    <Button>"Bottom Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::Bottom>
                <PopoverTrigger slot>
                    <Button>"Bottom"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover placement=PopoverPlacement::BottomEnd>
                <PopoverTrigger slot>
                    <Button>"Bottom End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
    </Grid>
}
```

### Tooltip

```rust demo
view! {
    <Space>
        <Popover tooltip=true>
            <PopoverTrigger slot>
                <Button>"Hover"</Button>
            </PopoverTrigger>
            "Content"
        </Popover>
    </Space>
}
```

### Popover Props

| Name         | Type                                | Default                    | Description                       |
| -------------| ----------------------------------- | -------------------------- | --------------------------------- |
| class        | `OptionalProp<MaybeSignal<String>>` | `Default::default()`       | Content class of the popover.     |
| placement    | `PopoverPlacement`                  | `PopoverPlacement::Top`    | Popover placement.                |
| trigger_type | `PopoverTriggerType`                | `PopoverTriggerType::Hover`| Action that displays the dropdown |
| tooltip      | `bool`                              | `false`                    | Tooltip.                          |
| children     | `Children`                          |                            | The content inside popover.       |

### Popover Slots

| Name           | Default | Description                                     |
| -------------- | ------- | ----------------------------------------------- |
| PopoverTrigger |         | The element or component that triggers popover. |

### PopoverTriger Props

| Name         | Type                                | Default                      | Description                                         |
| ------------ | ----------------------------------- | ---------------------------- | --------------------------------------------------  |
| class        | `OptionalProp<MaybeSignal<String>>` | `Default::default()`         | Addtional classes for the popover trigger element. |
| children     | `Children`                          |                              | The content inside popover trigger.                |

