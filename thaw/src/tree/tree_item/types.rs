use leptos::prelude::{expect_context, Memo};

#[derive(Debug, Default)]
pub enum TreeItemType {
    #[default]
    Leaf,
    Branch,
}

#[derive(Debug, Clone)]
pub(crate) struct TreeItemInjection {
    pub open: Memo<bool>,
}

impl TreeItemInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
