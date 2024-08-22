# Time Picker

```rust demo
use chrono::prelude::*;

let value = RwSignal::new(Local::now().time());
let option_value = RwSignal::new(Local::now().time());

view! {
    <Space vertical=true>
        <TimePicker value />
        <TimePicker value=option_value />
    </Space>
}
```

## TimePicker Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<TimePickerRule>` | `vec![]` | The rules to validate Field. |
| value | `OptionModel<NaiveTime>` | `Default::default()` | Set the TimePicker value. |
