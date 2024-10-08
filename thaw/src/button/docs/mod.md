# Button

```rust demo
view! {
    <Space>
        <Button>"Secondary"</Button>
        <Button appearance=ButtonAppearance::Primary>"Primary"</Button>
        <Button appearance=ButtonAppearance::Subtle>"Subtle"</Button>
        <Button appearance=ButtonAppearance::Transparent>"Transparent"</Button>
    </Space>
}
```

### Shape

```rust demo
view! {
    <Space>
        <Button>"Rounded"</Button>
        <Button shape=ButtonShape::Circular>"Circular"</Button>
        <Button shape=ButtonShape::Square>"Square"</Button>
    </Space>
}
```

### Icon

```rust demo
let icon = RwSignal::new(Some(icondata::AiCloseOutlined));

let on_click = move |_| {
    icon.update(|icon| {
        *icon = match icon {
            Some(data) => {
                if *data == icondata::AiCloseOutlined {
                    icondata::AiCheckOutlined
                } else {
                    icondata::AiCloseOutlined
                }
            }
            None => icondata::AiCloseOutlined
        }.into();
    });
};

view! {
    <Space vertical=true>
        <Space>
            <Button icon on_click>
                "Change icon"
            </Button>
            <Button icon on_click=move |_| icon.set(None)>
                "Clear icon"
            </Button>
        </Space>
        <Space>
            <Button icon=icondata::AiCloseOutlined>
                "Error Color Icon"
            </Button>
            <Button icon=icondata::AiCloseOutlined>
            </Button>
        </Space>
    </Space>
}
```

### Size

```rust demo
view! {
    <Space vertical=true>
        <Space>
            <Button size=ButtonSize::Small>"Small"</Button>
            <Button size=ButtonSize::Small icon=icondata::AiCloseOutlined>
                "Small with calendar icon"
            </Button>
            <Button size=ButtonSize::Small icon=icondata::AiCloseOutlined>
            </Button>
        </Space>
        <Space>
            <Button>"Medium"</Button>
            <Button icon=icondata::AiCloseOutlined>
                "Medium with calendar icon"
            </Button>
            <Button icon=icondata::AiCloseOutlined>
            </Button>
        </Space>
        <Space>
            <Button size=ButtonSize::Large>"Large"</Button>
            <Button size=ButtonSize::Large icon=icondata::AiCloseOutlined>
                "Large with calendar icon"
            </Button>
            <Button size=ButtonSize::Large icon=icondata::AiCloseOutlined>
            </Button>
        </Space>
    </Space>
}
```

### Disabled

```rust demo
view! {
    <Space vertical=true>
        <Space>
            <Button disabled=true>
                "Secondary"
            </Button>
            <Button appearance=ButtonAppearance::Primary disabled=true>
                "Primary"
            </Button>
            <Button appearance=ButtonAppearance::Subtle disabled=true>
                "Subtle"
            </Button>
            <Button appearance=ButtonAppearance::Transparent disabled=true>
                "Transparent"
            </Button>
        </Space>
        <Space>
            <Button disabled_focusable=true>
                "Secondary"
            </Button>
            <Button appearance=ButtonAppearance::Primary disabled_focusable=true>
                "Primary"
            </Button>
            <Button appearance=ButtonAppearance::Subtle disabled_focusable=true>
                "Subtle"
            </Button>
            <Button appearance=ButtonAppearance::Transparent disabled_focusable=true>
                "Transparent"
            </Button>
        </Space>
    </Space>
}
```

### Block

```rust demo
view! {
    <Button block=true>"Primary"</Button>
}
```

### Group

```rust demo
view! {
    <Space>
        <ButtonGroup>
            <Button>"Outlined"</Button>
            <Button>"Outlined"</Button>
            <Button>"Outlined"</Button>
        </ButtonGroup>
        <ButtonGroup vertical=true>
            <Button>"Outlined"</Button>
            <Button>"Outlined"</Button>
            <Button>"Outlined"</Button>
        </ButtonGroup>
    </Space>
}
```

### Button Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| appearance | `MaybeSignal<ButtonAppearance>` | `Default::default()` | A button can have its content and borders styled for greater emphasis or to be subtle. |
| shape | `MaybeSignal<ButtonShape>` | `Default::default()` | A button can be rounded, circular, or square. |
| size | `MaybeSignal<ButtonSize>` | `ButtonSize::Medium` | A button supports different sizes. |
| block | `MaybeSignal<bool>` | `false` | Whether the button is displayed as block. |
| icon | `MaybeProp<icondata_core::Icon>` | `None` | The icon of the button. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the button is disabled. |
| disabled_focusable | `MaybeSignal<bool>` | `false` | When set, allows the button to be focusable even when it has been disabled. |
| on_click | `Option<BoxOneCallback<ev::MouseEvent>>` | `None` | Listen for button click events. |
| children | `Option<Children>` |  |  |

### ButtonGroup props

| Name     | Type                | Default              | Description                         |
| -------- | ------------------- | -------------------- | ----------------------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                                     |
| vertical | `bool`              | `false`              | Directions of buttons in the group. |
| children | `Children`          |                      |                                     |
