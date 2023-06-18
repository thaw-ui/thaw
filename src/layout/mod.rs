mod layout_header;

use crate::utils::mount_style::mount_style;
pub use layout_header::*;
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
    cx: Scope,
    #[prop(optional, into)] style: MaybeSignal<String>,
    #[prop(optional)] position: LayoutPosition,
    children: Children,
) -> impl IntoView {
    let class_name = mount_style("layout", || style_sheet_str!("./src/layout/layout.css"));
    view! { cx, class=class_name,
        <div class="melt-layout" class=("melt-layout--absolute-positioned", position == LayoutPosition::ABSOLUTE) style=move || style.get()>
            { children(cx) }
        </div>
    }
}
