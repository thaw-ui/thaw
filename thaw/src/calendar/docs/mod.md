# Calendar

```rust demo
use chrono::prelude::*;
let value = RwSignal::new(Local::now().date_naive());
let option_value = RwSignal::new(Some(Local::now().date_naive()));

view! {
    <Space vertical=true>
        <Calendar value />
        <Calendar value=option_value let(date: &NaiveDate)>
            {date.year()}"-"{date.month()}"-"{date.day()}
        </Calendar>
    </Space>
}
```

### Calendar Props

| Name     | Type                          | Default              | Desciption     |
| -------- | ----------------------------- | -------------------- | -------------- |
| class    | `MaybeProp<String>`           | `Default::default()` |                |
| value    | `OptionModel<NaiveDate>`      | `Default::default()` | selected date. |
| children | `Option<CalendarChildrenFn>>` | `None`               | .              |
