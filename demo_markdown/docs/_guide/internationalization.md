# Internationalization

Some component, `Calendar` and `DatePicker`, can use another locale that the default. 

You can set the locale via the `locale` props of the `ConfigProvider`. If you need to change the locale, you can call the `LocaleConfig::use_rw_locale()` function to get a `RwSignal` of the locale.

Only the English and French locales are fully supported. For other locales, the "Today" button is not translated.

```rust demo
use chrono::prelude::*;
use Locale;
let value = RwSignal::new(Local::now().date_naive());

let locale = LocaleConfig::use_rw_locale();

view! {
    <Space vertical=true>
        <Space>
            <Button on_click=move |_| locale.set(Locale::en_US.into())>"set locale to en_US"</Button>
            <Button on_click=move |_| locale.set(Locale::fr_FR.into())>"set locale to fr_FR"</Button>
        </Space>
        <DatePicker value/>
        <Calendar value />
    </Space>
}
```
