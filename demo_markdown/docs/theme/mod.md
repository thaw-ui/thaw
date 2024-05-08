# Theme

### ThemeProvider

```rust demo
let theme = create_rw_signal(Theme::light());

view! {
    <ThemeProvider theme>
        <Card>
            <Space>
                <Button on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button on_click=move |_| theme.set(Theme::dark())>"Dark"</Button>
            </Space>
        </Card>
    </ThemeProvider>
}
```

### Customize Theme

```rust demo
let theme = create_rw_signal(Theme::light());
let on_customize_theme = move |_| {
    theme.update(|theme| {
        theme.common.color_primary = "#f5222d".to_string();
        theme.common.color_primary_hover = "#ff4d4f".to_string();
        theme.common.color_primary_active = "#cf1322".to_string();
    });
};

view! {
    <ThemeProvider theme>
        <Card>
            <Space>
                <Button on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button on_click=on_customize_theme>"Customize Theme"</Button>
            </Space>
        </Card>
    </ThemeProvider>
}
```

### ThemeProvider Props

| Name  | Type                      | Default              | Description |
| ----- | ------------------------- | -------------------- | ----------- |
| theme | `Option<RwSignal<Theme>>` | `Default::default()` | Theme.      |
