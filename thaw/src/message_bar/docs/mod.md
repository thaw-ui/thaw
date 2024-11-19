# MessageBar

Communicates important information about the state of the entire application or surface. For example, the status of a page, panel, dialog or card. The information shouldn't require someone to take immediate action, but should persist until the user performs one of the required actions.

### Default

```rust demo
view! {
    <MessageBar>
        <MessageBarBody>
            <MessageBarTitle>"Descriptive title"</MessageBarTitle>
            "Message providing information to the user with actionable insights."
        </MessageBarBody>
        <MessageBarActions>
            <Button>"Action"</Button>
            <Button>"Action"</Button>
            <MessageBarContainerAction slot>
                <Button
                    appearance=ButtonAppearance::Transparent
                    icon=icondata::AiCloseOutlined
                />
            </MessageBarContainerAction>
        </MessageBarActions>
    </MessageBar>
}
```

### Intent

```rust demo
view! {
    <Space vertical=true>
        <MessageBar>
            <MessageBarBody>
                <MessageBarTitle>"Intent info"</MessageBarTitle>
                "Message providing information to the user with actionable insights."
            </MessageBarBody>
        </MessageBar>
        <MessageBar intent=MessageBarIntent::Warning>
            <MessageBarBody>
                <MessageBarTitle>"Intent warning"</MessageBarTitle>
                "Message providing information to the user with actionable insights."
            </MessageBarBody>
        </MessageBar>
        <MessageBar intent=MessageBarIntent::Error>
            <MessageBarBody>
                <MessageBarTitle>"Intent error"</MessageBarTitle>
                "Message providing information to the user with actionable insights."
            </MessageBarBody>
        </MessageBar>
        <MessageBar intent=MessageBarIntent::Success>
            <MessageBarBody>
                <MessageBarTitle>"Intent success"</MessageBarTitle>
                "Message providing information to the user with actionable insights."
            </MessageBarBody>
        </MessageBar>
    </Space>
}
```

### Manual Layout

```rust demo
view! {
    <MessageBar layout=MessageBarLayout::Multiline>
        <MessageBarBody>
            <MessageBarTitle>"Descriptive title"</MessageBarTitle>
            "Message providing information to the user with actionable insights."
        </MessageBarBody>
        <MessageBarActions>
            <Button>"Action"</Button>
            <Button>"Action"</Button>
            <MessageBarContainerAction slot>
                <Button
                    appearance=ButtonAppearance::Transparent
                    icon=icondata::AiCloseOutlined
                />
            </MessageBarContainerAction>
        </MessageBarActions>
    </MessageBar>
}
```

### MessageBar Props

| Name     | Type                            | Default                        | Description                           |
| -------- | ------------------------------- | ------------------------------ | ------------------------------------- |
| class    | `MaybeProp<String>,`            | `Default::default()`           |                                       |
| layout   | `MessageBarLayout`              | `MessageBarLayout::Singleline` |                                       |
| intent   | `MaybeSignal<MessageBarIntent>` | `MessageBarIntent::Info`       | Default designs announcement presets. |
| children | `Children`                      |                                |                                       |

### MessageBarTitle Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### MessageBarBody Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### MessageBarActions Props

| Name                         | Type                                     | Default | Description |
| ---------------------------- | ---------------------------------------- | ------- | ----------- |
| message_bar_container_action | slot `Option<MessageBarContainerAction>` | `None`  |             |
| children                     | `Children`                               |         |             |

### MessageBarContainerAction Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |
