# Button

```rust demo
view! {
    <Space>
        <Button variant=ButtonVariant::Primary>"Primary"</Button>
        <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
        <Button variant=ButtonVariant::Text>"Text"</Button>
        <Button variant=ButtonVariant::Link>"Link"</Button>
    </Space>
}
```

### Color

```rust demo
view! {
    <Space>
        <Button color=ButtonColor::Primary>"Primary Color"</Button>
        <Button color=ButtonColor::Success>"Success Color"</Button>
        <Button color=ButtonColor::Warning>"Warning Color"</Button>
        <Button color=ButtonColor::Error>"Error Color"</Button>
    </Space>
}
```

### Icon

```rust demo
let icon = create_rw_signal(Some(icondata::AiCloseOutlined));

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
            <Button color=ButtonColor::Error icon=icondata::AiCloseOutlined>
                "Error Color Icon"
            </Button>
            <Button color=ButtonColor::Error icon=icondata::AiCloseOutlined>
                "Error Color Icon"
            </Button>
            <Button
                color=ButtonColor::Error
                icon=icondata::AiCloseOutlined
                round=true
            />
            <Button
                color=ButtonColor::Error
                icon=icondata::AiCloseOutlined
                circle=true
            />
        </Space>
    </Space>
}
```

### Loading

```rust demo
let loading = create_rw_signal(false);
let on_click = move |_| {
    loading.set(true);
    set_timeout(
        move || {
            loading.set(false);
        },
        std::time::Duration::new(2, 0),
    );
};

view! {
    <Space>
        <Button loading on_click icon=icondata::AiCloseOutlined>
            "Click Me"
        </Button>
        <Button loading on_click>
            "Click Me"
        </Button>
    </Space>
}
```

### Disabled

```rust demo
view! {
    <Space>
        <Button variant=ButtonVariant::Primary disabled=true>
            "Primary"
        </Button>
        <Button variant=ButtonVariant::Outlined disabled=true>
            "Outlined"
        </Button>
        <Button variant=ButtonVariant::Text disabled=true>
            "Text"
        </Button>
        <Button variant=ButtonVariant::Link disabled=true>
            "Link"
        </Button>
    </Space>
}
```

### Size

```rust demo
view! {
    <Space>
        <Button size=ButtonSize::Tiny>"Primary"</Button>
        <Button size=ButtonSize::Small>"Primary"</Button>
        <Button size=ButtonSize::Medium>"Primary"</Button>
        <Button size=ButtonSize::Large>"Primary"</Button>
    </Space>
}
```

### Group

```rust demo
view! {
    <Space>
        <ButtonGroup>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
        </ButtonGroup>
        <ButtonGroup vertical=true>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
            <Button variant=ButtonVariant::Outlined>"Outlined"</Button>
        </ButtonGroup>
    </Space>
}
```

### Button Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Additional classes for the button element. |
| style | `Option<MaybeSignal<String>>` | `Default::default()` | Button's style. |
| variant | `MaybeSignal<ButtonVariant>` | `ButtonVariant::Primary` | Button's variant. |
| color | `MaybeSignal<ButtonColor>` | `ButtonColor::Primary` | Button's color. |
| round | `MaybeSignal<bool>` | `false` | Whether the button shows rounded corners. |
| circle | `MaybeSignal<bool>` | `false` | Whether the button is round. |
| icon | `OptionalMaybeSignal<icondata_core::Icon>` | `None` | The icon of the button. |
| loading | `MaybeSignal<bool>` | `false` | Whether the button shows the loading status. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the button is disabled. |
| size | `MaybeSignal<ButtonSize>` | `ButtonSize::Medium` | Button size. |
| on_click | `Option<Callback<ev::MouseEvent>>` | `None` | Listen for button click events. |
| children | `Option<Children>` |  | Button's content. |

### ButtonGroup props

| Name     | Type       | Default | Description                         |
| -------- | ---------- | ------- | ----------------------------------- |
| vertical | `bool`     | `false` | Directions of buttons in the group. |
| children | `Children` |         | ButtonGroup's content.              |
