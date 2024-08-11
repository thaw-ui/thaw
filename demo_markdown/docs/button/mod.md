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
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Additional classes for the button element. |
| style | `Option<MaybeSignal<String>>` | `Default::default()` | Button's style. |
| appearance | `MaybeSignal<ButtonAppearance>` | `ButtonAppearance::Primary` | Button's variant. |
| color | `MaybeSignal<ButtonColor>` | `ButtonColor::Primary` | Button's color. |
| block | `MaybeSignal<bool>` | `false` | Whether the button is displayed as block. |
| round | `MaybeSignal<bool>` | `false` | Whether the button shows rounded corners. |
| circle | `MaybeSignal<bool>` | `false` | Whether the button is round. |
| icon | `OptionalMaybeSignal<icondata_core::Icon>` | `None` | The icon of the button. |
| loading | `MaybeSignal<bool>` | `false` | Whether the button shows the loading status. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the button is disabled. |
| size | `MaybeSignal<ButtonSize>` | `ButtonSize::Medium` | Button size. |
| on_click | `Option<Callback<ev::MouseEvent>>` | `None` | Listen for button click events. |
| children | `Option<Children>` |  | Button's content. |

### ButtonGroup props

| Name     | Type                                | Default              | Description                               |
| -------- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Additional classes for the group element. |
| vertical | `bool`                              | `false`              | Directions of buttons in the group.       |
| children | `Children`                          |                      | ButtonGroup's content.                    |
