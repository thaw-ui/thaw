# Calendar

```rust demo
use chrono::prelude::*;
let value = RwSignal::new(Local::now().date_naive());
let option_value = RwSignal::new(Some(Local::now().date_naive()));

view! {
    <Space vertical=true>
        <Calendar value />
        <Calendar value=option_value />
    </Space>
}
```

### Calendar Props

| Name  | Type                                | Default              | Desciption                                  |
| ----- | ----------------------------------- | -------------------- | ------------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the calendar element. |
| value | `Model<Option<NaiveDate>>`          | `Default::default()` | Set the calendar value                      |
