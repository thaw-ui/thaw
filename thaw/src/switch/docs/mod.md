# Switch

```rust demo
let checked = RwSignal::new(false);

view! {
    <Switch checked />
}
```

### Disabled

A Switch can be disabled.

```rust demo
view! {
    <Flex vertical=true inline=true>
        <Switch disabled=true label="Unchecked and disabled" />
        <Switch checked=true disabled=true label="Checked and disabled" />
    </Flex>
}
```

### Label

```rust demo
let checked = RwSignal::new(false);

view! {
    <Switch checked label="Label" />
}
```

### Switch Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| disabled | `Signal<bool>` | `false` | Whether to disable the switch. |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| value | `MaybeProp<String>` | `Default::default()` | A string specifying the value for the input control. |
| rules | `Vec<SwitchRule>` | `vec![]` | The rules to validate Field. |
| checked | `Model<bool>` | `false` | Defines the controlled checked state of the Switch. |
| label | `MaybeProp<String>` | `Default::default()` | The Switch's label. |
