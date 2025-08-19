# Time Picker

```rust demo
use chrono::prelude::*;

let value = RwSignal::new(Local::now().time());
let option_value = RwSignal::new(Local::now().time());

view! {
    <Flex vertical=true inline=true>
        <TimePicker value />
        <TimePicker value=option_value />
    </Flex>
}
```

### Disabled

```rust demo
use chrono::prelude::*;
let value = RwSignal::new(Local::now().time());

view! {
    <Flex vertical=true inline=true>
        <TimePicker disabled=true />
        <TimePicker value disabled=true />
    </Flex>
}
```

### Size

```rust demo
view! {
    <Flex vertical=true inline=true>
        <TimePicker size=TimePickerSize::Small/>
        <TimePicker />
        <TimePicker size=TimePickerSize::Large/>
    </Flex>
}
```

## TimePicker Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| disabled | `Signal<bool>` | `false` | Whether time picker is disabled. |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<TimePickerRule>` | `vec![]` | The rules to validate Field. |
| value | `OptionModel<NaiveTime>` | `Default::default()` | Set the TimePicker value. |
| size | `Signal<TimePickerSize>` | `TimePickerSize::Medium` | Size of the input. |
