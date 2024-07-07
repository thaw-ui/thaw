# Calendar

```rust demo
use chrono::prelude::*;
let value = RwSignal::new(Some(Local::now().date_naive()));

view! {
    <Calendar value />
}
```

### Calendar Props

| Name  | Type                                | Default              | Desciption                                  |
| ----- | ----------------------------------- | -------------------- | ------------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the calendar element. |
| value | `Model<Option<NaiveDate>>`          | `Default::default()` | Set the calendar value                      |
