use leptos::prelude::*;
use thaw_utils::Model;

#[derive(Clone, Copy)]
pub(crate) struct RatingInjection {
    pub value: Model<f32>,
    pub hovered_value: RwSignal<Option<f32>>,
    pub name: StoredValue<String>,
    pub step: Signal<f32>,
    pub color: Signal<RatingColor>,
}

impl RatingInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Default, Clone)]
pub enum RatingColor {
    Brand,
    // TODO v0.5 Marigold,
    #[default]
    Neutral,
}

impl RatingColor {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Brand => "brand",
            // RatingColor::Marigold => "marigold",
            Self::Neutral => "neutral",
        }
    }
}
