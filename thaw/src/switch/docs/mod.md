# Switch

```rust demo
let checked = RwSignal::new(false);

view! {
    <Switch checked />
}
```

### Switch Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| value | `MaybeProp<String>` | `Default::default()` | A string specifying the value for the input control. |
| rules | `Vec<SwitchRule>` | `vec![]` | The rules to validate Field. |
| checked | `Model<bool>` | `false` | Defines the controlled checked state of the Switch. |
| label | `MaybeProp<String>` | `Default::default()` | The Switch's label. |
