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
let page = RwSignal::new(1);
let selected_value = RwSignal::new(String::from("apple"));

view! {
    <ConfigProvider theme>
        <Card>
            <Space>
                <Button appearance=ButtonAppearance::Primary on_click=move |_| theme.set(Theme::light())>"Light"</Button>
                <Button appearance=ButtonAppearance::Primary on_click=on_customize_light_theme>"Custom Light Theme"</Button>
                <Button appearance=ButtonAppearance::Primary on_click=on_customize_dark_theme>"Custom Dark Theme"</Button>
            </Space>
            <Link href="https://react.fluentui.dev/?path=/docs/theme-theme-designer--docs">
                "You can use this tool to generate brand color palette"
            </Link>
            <TabList selected_value>
                <Tab value="apple">
                    "Apple"
                </Tab>
                <Tab value="pear">
                    "Pear"
                </Tab>
            </TabList>
            <RadioGroup value=selected_value>
                <Radio value="apple" label="Apple"/>
                <Radio value="pear" label="Pear"/>
            </RadioGroup>
            <InfoLabel>
                <InfoLabelInfo slot>
                    "This is example information for an InfoLabel. "
                </InfoLabelInfo>
                "Example label"
            </InfoLabel>
            <Badge appearance=BadgeAppearance::Filled>"10+"</Badge>
            <Checkbox />
            <Switch />
            <Tag dismissible=true>"Tag"</Tag> //TODO make dismissable
            <Input/>
            <Select>
                <option>"Red"</option>
                <option>"Green"</option>
                <option>"Blue"</option>
            </Select>
            <TimePicker />
            <DatePicker/>
            <Slider step=25.0 value/>
            <ProgressBar value/>
            <Pagination page page_count=10 />
        </Card>
    </ConfigProvider>
}
```
