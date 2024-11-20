# InfoLabel

An InfoLabel is a Label with an InfoButton at the end.

```rust demo
view! {
    <InfoLabel>
        <InfoLabelInfo slot>
            "This is example information for an InfoLabel. "
        </InfoLabelInfo>
        "Example label"
    </InfoLabel>
}
```

### Required

```rust demo
view! {
    <InfoLabel required=true>
        <InfoLabelInfo slot>
            "This is example information for an InfoLabel. "
        </InfoLabelInfo>
        "Required label"
    </InfoLabel>
}
```

### Size

```rust demo
view! {
    <Flex vertical=true>
        <InfoLabel size=InfoLabelSize::Small>
            <InfoLabelInfo slot>
                "This is example information for an InfoLabel. "
            </InfoLabelInfo>
            "Small label"
        </InfoLabel>
        <InfoLabel>
            <InfoLabelInfo slot>
                "This is example information for an InfoLabel. "
            </InfoLabelInfo>
            "Medium label"
        </InfoLabel>
        <InfoLabel size=InfoLabelSize::Large>
            <InfoLabelInfo slot>
                "This is example information for an InfoLabel. "
            </InfoLabelInfo>
            "Large label"
        </InfoLabel>
    </Flex>
}
```

### InfoLabel Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| size | `Signal<InfoLabelSize>` | `LabelSize::Medium` | A label supports different sizes. |
| weight | `Signal<InfoLabelWeight>` | `LabelWeight::Regular` | A label supports regular and semibold fontweight. |
| disabled | `Signal<bool>` | `false` | Renders the label as disabled. |
| required | `Signal<bool>` | `false` | Displays an indicator that the label is for a required field. |
| info_label_info | slot `InfoLabelInfo` |  |  |
| children | `Children` |  |  |

### InfoLabelInfo Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |
