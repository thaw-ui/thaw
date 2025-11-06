# Button

A button triggers an action or event when activated.

### Default

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

### Loading

```rust demo
let loading = RwSignal::new(false);
let on_click = move |_| {
    loading.set(true);
    set_timeout(
        move || {
            loading.set(false);
        },
        std::time::Duration::from_secs(5),
    );
};

view! {
    <Space>
        <Button loading on_click icon=icondata::AiCloseOutlined>
            "Start loading"
        </Button>
        <Button loading on_click>
            "Start loading"
        </Button>
        <Button loading on_click size=ButtonSize::Small>
            "Start loading"
        </Button>
        <Button loading on_click size=ButtonSize::Large>
            "Start loading"
        </Button>
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

### Imperative handle

```rust demo
let count = RwSignal::new(0);
let on_click = move |_| *count.write() += 1;

let button_ref = ComponentRef::<ButtonRef>::new();

let click = move |_| {
    button_ref.get_untracked().unwrap().click()
};

let focus = move |_| {
    button_ref.get_untracked().unwrap().focus()
};

view! {
    <Space>
        <Button on_click=click>"Click"</Button>
        <Button on_click=focus>"Focus"</Button>
        <Button comp_ref=button_ref on_click>{count}</Button>
    </Space>
}
```

### Button Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| appearance | `Signal<ButtonAppearance>` | `Default::default()` | A button can have its content and borders styled for greater emphasis or to be subtle. |
| shape | `Signal<ButtonShape>` | `Default::default()` | A button can be rounded, circular, or square. |
| size | `Option<Signal<ButtonSize>>` | `ButtonSize::Medium` | A button supports different sizes. |
| block | `Signal<bool>` | `false` | Whether the button is displayed as block. |
| icon | `MaybeProp<icondata_core::Icon>` | `None` | The icon of the button. |
| disabled | `Signal<bool>` | `false` | Whether the button is disabled. |
| disabled_focusable | `Signal<bool>` | `false` | When set, allows the button to be focusable even when it has been disabled. |
| loading | `Signal<bool>` | `false` | Whether the button shows the loading status. |
| on_click | `Option<BoxOneCallback<ev::MouseEvent>>` | `None` | Listen for button click events. |
| children | `Option<Children>` |  |  |
| comp_ref | ref `ComponentRef<ButtonRef>` | `Default::default()` |  |

### ButtonGroup props

| Name     | Type                | Default              | Description                         |
| -------- | ------------------- | -------------------- | ----------------------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                                     |
| vertical | `bool`              | `false`              | Directions of buttons in the group. |
| children | `Children`          |                      |                                     |

### ButtonRef Props

| Name  | Type        | Description               |
| ----- | ----------- | ------------------------- |
| click | `Fn(&self)` | Click the button element. |
| focus | `Fn(&self)` | Focus the button element. |
| blur  | `Fn(&self)` | Blurs the button element. |
