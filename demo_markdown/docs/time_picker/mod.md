# Time Picker

```rust demo
use thaw::chrono::prelude::*;

let value = create_rw_signal(Some(Local::now().time()));

view! {
    <TimePicker value />
}
```

## TimePicker Props

| Name  | Type                          | Default              | Description                                    |
| ----- | ----------------------------- | -------------------- | ---------------------------------------------- |
| class | `MaybeSignal<String>`         | `Default::default()` | Addtional classes for the time picker element. |
| value | `RwSignal<Option<NaiveTime>>` | `Default::default()` | Set the TimePicker value.                      |
