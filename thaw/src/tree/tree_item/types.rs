use leptos::{
    html,
    prelude::{expect_context, Memo, NodeRef},
};

#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub enum TreeItemType {
    #[default]
    Leaf,
    Branch,
}

impl TreeItemType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Leaf => "leaf",
            Self::Branch => "branch",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct TreeItemInjection {
    pub open: Memo<bool>,
    pub item_type: TreeItemType,
    pub subtree_ref: NodeRef<html::Div>,
}

impl TreeItemInjection {
    pub fn expect_context() -> Self {
        expect_context()
    }
}
