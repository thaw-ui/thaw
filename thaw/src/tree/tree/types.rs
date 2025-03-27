use leptos::prelude::{expect_context, use_context};
use std::collections::HashSet;
use thaw_utils::Model;

#[derive(Clone)]
pub(crate) struct TreeInjection {
    pub open_items: Model<HashSet<String>>,
}

impl TreeInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}

#[derive(Clone)]
pub(crate) struct SubtreeInjection {
    pub level: usize,
}

impl SubtreeInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }

    pub fn use_context() -> Option<Self> {
        use_context()
    }
}
