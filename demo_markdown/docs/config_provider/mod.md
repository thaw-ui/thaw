# ConfigProvider

### Theme

```rust demo
let theme = RwSignal::new(Theme::light());

view! {
    <ConfigProvider theme>
        <Card>
            <Space>
                <Button on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button on_click=move |_| theme.set(Theme::dark())>"Dark"</Button>
            </Space>
        </Card>
    </ConfigProvider>
}
```

### Customize Theme

```rust demo
let theme = RwSignal::new(Theme::light());
let on_customize_theme = move |_| {
    theme.update(|theme| {
        theme.color.color_brand_background = "#f5222d".to_string();
        theme.color.color_brand_background_hover = "#ff4d4f".to_string();
        theme.color.color_brand_background_pressed = "#cf1322".to_string();
    });
};

view! {
    <ConfigProvider theme>
        <Card>
            <Space>
                <Button appearance=ButtonAppearance::Primary on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button appearance=ButtonAppearance::Primary on_click=on_customize_theme>"Customize Theme"</Button>
            </Space>
        </Card>
    </ConfigProvider>
}
```

### ConfigProvider Props

| Name  | Type                      | Default              | Description |
| ----- | ------------------------- | -------------------- | ----------- |
| theme | `Option<RwSignal<Theme>>` | `Default::default()` | Theme.      |
