# Date Picker

```rust demo
use chrono::prelude::*;
let value = create_rw_signal(Some(Local::now().date_naive()));

view! {
    <DatePicker value/>
}
```

### DatePicker Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the date picker element. |
| value | `Model<Option<NaiveDate>>` | `Default::default()` | Set the date picker value |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |
