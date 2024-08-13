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

| Name  | Type                     | Default              | Description               |
| ----- | ------------------------ | -------------------- | ------------------------- |
| class | `MaybeProp<String>`      | `Default::default()` |                           |
| value | `OptionModel<NaiveTime>` | `Default::default()` | Set the TimePicker value. |
