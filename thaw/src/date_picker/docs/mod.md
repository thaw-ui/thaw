# Date Picker

```rust demo
use chrono::prelude::*;
let value = RwSignal::new(Local::now().date_naive());
let option_value = RwSignal::new(Some(Local::now().date_naive()));

view! {
    <Space vertical=true>
        <DatePicker value/>
        <DatePicker value=option_value/>
    </Space>
}
```

### Size

```rust demo
view! {
    <Flex vertical=true>
        <DatePicker size=DatePickerSize::Small/>
        <DatePicker />
        <DatePicker size=DatePickerSize::Large/>
    </Flex>
}
```

### DatePicker Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<DatePickerRule<T>>` | `vec![]` | The rules to validate Field. |
| value | `OptionModel<NaiveDate>` | `Default::default()` | Set the date picker value. |
| size | `Signal<DatePickerSize>` | `DatePickerSize::Medium` | Size of the input. |
