# Calendar

```rust demo
use chrono::prelude::*;
let value = create_rw_signal(Some(Local::now().date_naive()));

view! {
    <Calendar value />
}
```

### Calendar Props

| Name  | Type                          | Default              | Desciption                                  |
| ----- | ----------------------------- | -------------------- | ------------------------------------------- |
| class | `MaybeSignal<String>`         | `Default::default()` | Addtional classes for the calendar element. |
| value | `RwSignal<Option<NaiveDate>>` | `Default::default()` | Set the calendar value                      |
