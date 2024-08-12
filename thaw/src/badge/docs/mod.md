# Badge

```rust demo
view! {
    <Badge />
}
```

### Appearance

```rust demo
view! {
    <Space>
        <Badge appearance=BadgeAppearance::Filled>"999+"</Badge>
        <Badge appearance=BadgeAppearance::Ghost>"999+"</Badge>
        <Badge appearance=BadgeAppearance::Outline>"999+"</Badge>
        <Badge appearance=BadgeAppearance::Tint>"999+"</Badge>
    </Space>
}
```

### Sizes

```rust demo
view! {
    <Space>
        <Badge size=BadgeSize::Tiny/>
        <Badge size=BadgeSize::ExtraSmall/>
        <Badge size=BadgeSize::Small/>
        <Badge size=BadgeSize::Medium/>
        <Badge size=BadgeSize::Large/>
        <Badge size=BadgeSize::ExtraLarge/>
    </Space>
}
```

### Color

```rust demo
view! {
    <Space vertical=true>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Brand>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Brand>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Brand>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Brand>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Danger>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Danger>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Danger>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Danger>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Important>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Important>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Important>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Important>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Informative>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Informative>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Informative>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Informative>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Severe>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Severe>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Severe>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Severe>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Subtle>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Subtle>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Subtle>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Subtle>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Success>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Success>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Success>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Success>"999+"</Badge>
        </Space>
        <Space>
            <Badge appearance=BadgeAppearance::Filled color=BadgeColor::Warning>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Ghost color=BadgeColor::Warning>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Outline color=BadgeColor::Warning>"999+"</Badge>
            <Badge appearance=BadgeAppearance::Tint color=BadgeColor::Warning>"999+"</Badge>
        </Space>
    </Space>
}
```

### Badge Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| appearance | `MaybeSignal<BadgeAppearance>` | `Default::default()` | A Badge can be filled, outline, ghost, inverted. |
| size | `MaybeSignal<BadgeSize>` | `Default::default()` | A Badge can be on of several preset sizes. |
| color | `MaybeSignal<BadgeColor>` | `Default::default()` | A Badge can be one of preset colors. |
| children | `Option<Children>` | `None` |  |
