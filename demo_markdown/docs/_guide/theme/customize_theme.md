# Customize Theme

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

### Custom Brand Color

```rust demo
use std::collections::HashMap;

let brand_colors = RwSignal::new(HashMap::from([
    (10, "#050201"),
    (20, "#231310"),
    (30, "#3B1C19"),
    (40, "#4F231F"),
    (50, "#642A26"),
    (60, "#7A322C"),
    (70, "#903A32"),
    (80, "#A64139"),
    (90, "#BD4A3F"),
    (100, "#D45246"),
    (110, "#EC5B4D"),
    (120, "#FB6D5B"),
    (130, "#FF8571"),
    (140, "#FF9E8B"),
    (150, "#FFB5A4"),
    (160, "#FFCABE")
]));


let theme = RwSignal::new(Theme::light());
let on_customize_light_theme = move |_| {
    theme.set(Theme::custom_light(&brand_colors.get()));
};
let on_customize_dark_theme = move |_| {
    theme.set(Theme::custom_dark(&brand_colors.get()));
};
let value = RwSignal::new(0.0);

view! {
    <ConfigProvider theme>
        <Card>
            <Space>
                <Button appearance=ButtonAppearance::Primary on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button appearance=ButtonAppearance::Primary on_click=on_customize_light_theme>"Custom Light Theme"</Button>
                <Button appearance=ButtonAppearance::Primary on_click=on_customize_dark_theme>"Custom Dark Theme"</Button>
            </Space>
            <Input/>
            <Link href="https://react.fluentui.dev/?path=/docs/theme-theme-designer--docs">
                "You can use this tool to generate brand color palette"
            </Link>
            <Checkbox />
            <Badge appearance=BadgeAppearance::Filled>"10+"</Badge>
            <Slider step=25.0 value/>
        </Card>
    </ConfigProvider>
}
```
