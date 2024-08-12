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

### DatePicker Props

| Name  | Type                     | Default              | Desciption                 |
| ----- | ------------------------ | -------------------- | -------------------------- |
| class | `MaybeProp<String>`      | `Default::default()` |                            |
| value | `OptionModel<NaiveDate>` | `Default::default()` | Set the date picker value. |
