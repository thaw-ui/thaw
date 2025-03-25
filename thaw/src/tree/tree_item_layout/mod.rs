use crate::{ChevronRight12RegularIcon, ConfigDirection, ConfigInjection, TreeItemInjection};
use leptos::prelude::*;
use thaw_utils::class_list;

const DURATION_NORMAL: &'static str = "200ms";
const CURVE_EASY_EASE_MAX: &'static str = "cubic-bezier(0.8, 0, 0.2, 1)";

#[component]
pub fn TreeItemLayout(children: Children) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-tree-item-layout"]>
            <div class="thaw-tree-item-layout__expand-icon">
                <TreeItemChevron />
            </div>
            <div class="thaw-tree-item-layout__main">{children()}</div>
        </div>
    }
}

#[component]
pub fn TreeItemChevron() -> impl IntoView {
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

        format!("{expand_icon_inline_style}; transition: transform {DURATION_NORMAL}ms {CURVE_EASY_EASE_MAX};")
    };

    view! { <ChevronRight12RegularIcon attr:style=style /> }
}
