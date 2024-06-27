# MessageBar

```rust demo
view! {
    <MessageBar>
        <MessageBarBody>
            <MessageBarTitle>Descriptive title</MessageBarTitle>
            "Message providing information to the user with actionable insights."
        </MessageBarBody>
        <MessageBarActions>
            <Button size=ButtonSize::Small>"Action"</Button>
            <Button size=ButtonSize::Small>"Action"</Button>
            <MessageBarContainerAction slot>
                <Button size=ButtonSize::Small appearance=ButtonAppearance::Transparent>
                    "X"
                </Button>
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
          <h3 style="margin: 0">"Descriptive title"</h3>
          <p>"Message providing information to the user with actionable insights."</p>
        </MessageBarBody>
    </MessageBar>
}
```
