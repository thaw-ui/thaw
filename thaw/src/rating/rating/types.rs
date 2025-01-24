use leptos::prelude::*;
use thaw_utils::Model;

#[derive(Clone, Copy)]
pub(crate) struct RatingInjection {
    pub value: Model<f32>,
    pub hovered_value: RwSignal<Option<f32>>,
    pub name: StoredValue<String>,
}

impl RatingInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}