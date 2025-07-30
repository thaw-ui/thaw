# Internationalization

Some component, `Calendar` and `DatePicker`, can use another locale that the default. 

You can use one of the provided locales:
```rust
use thaw::locales;
let locale = RwSignal::new(LocaleConfig::from(locales::EnUS));
```

Or you can create your own struct that implement the `LocaleExt` trait:
```rust
use thaw::locales::LocaleExt;
pub struct MyStruct;
impl LocaleExt for MyStruct {
    fn locale(&self) -> &Locale {todo!()}
    fn today(&self) -> &'static str {todo!()}
}
let locale = RwSignal::new(LocaleConfig::from(MyStruct));
```

When you have your `RwSignal<LocaleConfig>`, you can pass it to the `ConfigProvider` via the
`locale` props:
```rust
view! {
    <ConfigProvider locale>
        ...
    </ConfigProvider>
}
```

If you need to change the locale, but don't have access to the `RwSignal` you passed to the `ConfigProvider`, you can call the `LocaleConfig::use_rw_locale()` function to get a `RwSignal` of the locale.

Here is a demo of it working:
```rust demo
use chrono::prelude::*;
use Locale;
let value = RwSignal::new(Local::now().date_naive());

let locale = LocaleConfig::use_rw_locale();

view! {
    <Space vertical=true>
        <Space>
            <Button on_click=move |_| locale.set(locales::EnUS.into())>"set locale to en_US"</Button>
            <Button on_click=move |_| locale.set(locales::FrFR.into())>"set locale to fr_FR"</Button>
        </Space>
        <DatePicker value/>
        <Calendar value />
    </Space>
}
```
