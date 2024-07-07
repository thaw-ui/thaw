# Time Picker

```rust demo
use chrono::prelude::*;

let value = RwSignal::new(Some(Local::now().time()));

view! {
    <TimePicker value />
}
```

## TimePicker Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the time picker element. |
| value | `Model<Option<NaiveTime>>` | `Default::default()` | Set the TimePicker value. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |
