use std::sync::Arc;

use crate::ConfigInjection;
use chrono::Weekday;
use leptos::prelude::*;
use pure_rust_locales::locale_match;

pub use pure_rust_locales::Locale;

pub mod locales;

use locales::LocaleExt;
#[derive(Clone)]
pub struct LocaleConfig(Arc<dyn LocaleExt + Send + Sync>);

impl<T: LocaleExt + Send + Sync + 'static> From<T> for LocaleConfig {
    fn from(value: T) -> Self {
        Self(Arc::new(value))
    }
}

impl Default for LocaleConfig {
    fn default() -> Self {
        LocaleConfig::from(locales::EnUS)
    }
}

impl LocaleConfig {
    pub fn use_locale() -> ReadSignal<LocaleConfig> {
        use_context::<ConfigInjection>()
            .map_or_else(|| RwSignal::new(LocaleConfig::default()), |c| c.locale)
            .split()
            .0
    }

    pub fn use_rw_locale() -> RwSignal<LocaleConfig> {
        expect_context::<ConfigInjection>().locale
    }

    pub fn locale(&self) -> Locale {
        self.0.locale()
    }

    pub fn months(&self) -> &'static [&'static str] {
        locale_match!(self.locale() => LC_TIME::MON)
    }

    pub fn month(&self, month: u8) -> &'static str {
        self.months()[(month - 1) as usize]
    }

    pub fn ab_months(&self) -> &'static [&'static str] {
        locale_match!(self.locale() => LC_TIME::ABMON)
    }

    pub fn ab_month(&self, month: u8) -> &'static str {
        self.ab_months()[(month - 1) as usize]
    }

    pub fn ab_days(&self) -> &'static [&'static str] {
        locale_match!(self.locale() => LC_TIME::ABDAY)
    }

    /// day is the number of day from sunday (sunday=0, monday=1, ...)
    pub fn ab_day(&self, day: u8) -> &'static str {
        self.ab_days()[day as usize]
    }

    pub fn ab_weekday(&self, day: Weekday) -> &'static str {
        self.ab_day(((day as u8) + 1) % 7)
    }

    /// Return the first day of the week
    pub fn first_weekday(&self) -> Weekday {
        // the `LC_TIME::FIRST_WEEKDAY` value start at 1 for sunday, 2 for monday, ...
        // the `Weekday` values start at 0 for monday, 1 for tuesday, ...
        // this little operation convert between the two
        let number_of_days_since_mondays =
            ((locale_match!(self.locale() => LC_TIME::FIRST_WEEKDAY).unwrap_or(1) + 5) % 7) as u8;
        number_of_days_since_mondays.try_into().unwrap()
    }

    pub fn today(&self) -> &'static str {
        self.0.today()
    }
}
