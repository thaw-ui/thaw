use crate::ConfigInjection;
use leptos::prelude::*;
use pure_rust_locales::{locale_match, Locale};

#[derive(Clone, Default)]
pub struct LocaleConfig {
    pub locale: Locale,
}

impl From<Locale> for LocaleConfig {
    fn from(value: Locale) -> Self {
        LocaleConfig { locale: value }
    }
}

impl LocaleConfig {
    pub fn use_locale() -> ReadSignal<LocaleConfig> {
        use_context::<ConfigInjection>()
            .map_or_else(|| RwSignal::new(LocaleConfig::default()), |c| c.locale)
            .split()
            .0
    }

    pub fn months(&self) -> &'static [&'static str] {
        locale_match!(self.locale => LC_TIME::MON)
    }

    pub fn month(&self, month: u32) -> &'static str {
        self.months()[month as usize]
    }
}
