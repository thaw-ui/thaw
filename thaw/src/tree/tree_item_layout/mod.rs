use crate::{
    ChevronRight12RegularIcon, ConfigDirection, ConfigInjection, TreeItemInjection, TreeItemType,
};
use leptos::{either::Either, prelude::*};
use thaw_utils::class_list;

#[component]
pub fn TreeItemLayout(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let tree_item_injection = TreeItemInjection::expect_context();
    let is_branch = tree_item_injection.item_type == TreeItemType::Branch;

    view! {
        <div class=class_list![
            "thaw-tree-item-layout",
            class
        ]>
            {if is_branch {
                Either::Left(
                    view! {
                        <div class="thaw-tree-item-layout__expand-icon">
                            <TreeItemChevron />
                        </div>
                    },
                )
            } else {
                Either::Right(())
            }} <div class="thaw-tree-item-layout__main">{children()}</div>
        </div>
    }
}

#[component]
fn TreeItemChevron() -> impl IntoView {
    let tree_item_injection = TreeItemInjection::expect_context();
    let open = tree_item_injection.open;

    let config_injection = ConfigInjection::expect_context();

    let style = move || {
        let expand_icon_inline_style = if open.get() {
            "transform: rotate(90deg)"
        } else if config_injection.dir.map(|d| d.get()) != Some(ConfigDirection::Rtl) {
            "transform: rotate(0deg)"
        } else {
            "transform: rotate(180deg)"
        };

        format!("{expand_icon_inline_style}; transition: transform var(--durationNormal) var(--curveEasyEaseMax);")
    };

    view! { <ChevronRight12RegularIcon attr:style=style /> }
}
