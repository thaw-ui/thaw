# Calendar

```rust demo
use chrono::prelude::*;
use Locale;
let value = RwSignal::new(Local::now().date_naive());
let option_value = RwSignal::new(Some(Local::now().date_naive()));

let locale = LocaleConfig::use_rw_locale();

view! {
    <Space vertical=true>
        <Calendar value />
        <Calendar value=option_value let(date: &NaiveDate)>
            {date.year()}"-"{date.month()}"-"{date.day()}
        </Calendar>
        <Button on_click=move |_| locale.set(Locale::en_US.into())>"en_US locale"</Button>
        <Button on_click=move |_| locale.set(Locale::fr_FR.into())>"fr_FR locale"</Button>
    </Space>
}
```

### Calendar Props

| Name     | Type                          | Default              | Desciption     |
| -------- | ----------------------------- | -------------------- | -------------- |
| class    | `MaybeProp<String>`           | `Default::default()` |                |
| value    | `OptionModel<NaiveDate>`      | `Default::default()` | selected date. |
| children | `Option<CalendarChildrenFn>>` | `None`               | .              |
