mod layout_header;
mod layout_sider;

use crate::utils::mount_style::mount_style;
pub use layout_header::*;
pub use layout_sider::*;
use leptos::*;
use stylers::style_sheet_str;

#[derive(Default, PartialEq)]
pub enum LayoutPosition {
    #[default]
    STATIC,
    ABSOLUTE,
}

impl LayoutPosition {
    pub fn as_str(&self) -> &str {
        match self {
            LayoutPosition::STATIC => "static",
            LayoutPosition::ABSOLUTE => "absolute",
        }
    }
}

#[component]
pub fn Layout(
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(optional)] position: LayoutPosition,
    #[prop(optional, into)] has_sider: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    let class_name = mount_style("layout", || style_sheet_str!("./src/layout/layout.css"));

    let style = create_memo(move |_| {
        let mut style = style.get();
        if has_sider.get() {
            style.push_str("display: flex; flex-wrap: nowrap; flex-direction: row; width: 100%;")
        }
        style
    });
    view! {  class=class_name,
        <div class="melt-layout" class=("melt-layout--absolute-positioned", position == LayoutPosition::ABSOLUTE) style=move || style.get()>
            { children() }
        </div>
    }
}
