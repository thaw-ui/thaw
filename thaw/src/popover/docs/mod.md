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
            <Popover position=PopoverPosition::TopStart>
                <PopoverTrigger slot>
                    <Button>"Top Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::Top>
                <PopoverTrigger slot>
                    <Button>"Top"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::TopEnd>
                <PopoverTrigger slot>
                    <Button>"Top End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::LeftStart trigger_type=PopoverTriggerType::Click>
                <PopoverTrigger slot>
                    <Button>"Left Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover position=PopoverPosition::RightStart>
                <PopoverTrigger slot>
                    <Button>"Right Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::Left trigger_type=PopoverTriggerType::Click>
                <PopoverTrigger slot>
                    <Button>"Left"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover position=PopoverPosition::Right>
                <PopoverTrigger slot>
                    <Button>"Right"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::LeftEnd>
                <PopoverTrigger slot>
                    <Button>"Left End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem offset=1>
            <Popover position=PopoverPosition::RightEnd>
                <PopoverTrigger slot>
                    <Button>"Right End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::BottomStart>
                <PopoverTrigger slot>
                    <Button>"Bottom Start"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::Bottom>
                <PopoverTrigger slot>
                    <Button>"Bottom"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
        <GridItem>
            <Popover position=PopoverPosition::BottomEnd>
                <PopoverTrigger slot>
                    <Button>"Bottom End"</Button>
                </PopoverTrigger>
                "Content"
            </Popover>
        </GridItem>
    </Grid>
}
```

### Appearance

```rust demo
view! {
    <Space>
        <Popover appearance=PopoverAppearance::Brand>
            <PopoverTrigger slot>
                <Button>"Hover"</Button>
            </PopoverTrigger>
            "Content"
        </Popover>
        <Popover appearance=PopoverAppearance::Inverted>
            <PopoverTrigger slot>
                <Button>"Hover"</Button>
            </PopoverTrigger>
            "Content"
        </Popover>
    </Space>
}
```

### Popover Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| position | `PopoverPosition` | `PopoverPosition::Top` | Configures the position of the Popover. |
| appearance | `MaybeProp<PopoverAppearance>` | `Default::default()` | A popover can appear styled with brand or inverted. When not specified, the default style is used. |
| trigger_type | `PopoverTriggerType` | `PopoverTriggerType::Hover` | Action that displays the popover. |
| popover_trigger | slot `PopoverTrigger` |  | The element or component that triggers popover. |
| children | `Children` |  |  |

### PopoverTrigger Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
