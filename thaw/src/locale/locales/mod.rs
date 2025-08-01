use pure_rust_locales::Locale;

pub trait LocaleExt {
    fn locale(&self) -> Locale;
    fn today(&self) -> &'static str;
}

pub struct EnUS;
impl LocaleExt for EnUS {
    fn locale(&self) -> Locale {
        Locale::en_US
    }
    fn today(&self) -> &'static str {
        "Today"
    }
}

pub struct FrFR;
impl LocaleExt for FrFR {
    fn locale(&self) -> Locale {
        Locale::fr_FR
    }
    fn today(&self) -> &'static str {
        "Aujourd'hui"
    }
}
