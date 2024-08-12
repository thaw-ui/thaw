mod layout_header;
mod layout_sider;

pub use layout_header::*;
pub use layout_sider::*;

use crate::Scrollbar;
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[derive(Default, PartialEq)]
pub enum LayoutPosition {
    #[default]
    Static,
    Absolute,
}

impl LayoutPosition {
    pub fn as_str(&self) -> &str {
        match self {
            LayoutPosition::Static => "static",
            LayoutPosition::Absolute => "absolute",
        }
    }
}

#[component]
pub fn Layout(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Addtional classes for the layout element.
    #[prop(optional, into)] content_class: MaybeProp<String>,
    /// Style of scrollable content node.
    #[prop(optional, into)] content_style: MaybeProp<String>,
    /// static position will make it css position set to static.
    /// absolute position will make it css position set to absolute and left,
    /// right, top, bottom to 0. absolute position is very useful
    /// when you want to make content scroll in a fixed container or make
    /// the whole page's layout in a fixed position. You may need to change
    /// the style of the component to make it display as you expect.
    #[prop(optional)] position: LayoutPosition,
    /// Whether the component has sider inside. If so it must be true.
    #[prop(optional, into)] has_sider: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("layout", include_str!("./layout.css"));

    let sider_style = Memo::new(move |_| {
        if has_sider.get() {
            Some("display: flex; flex-wrap: nowrap; flex-direction: row; width: 100%;")
        } else {
            None
        }
    });
    view! {
        <div class=class_list![gen_class(position), class]>
            <Scrollbar
                content_class
                content_style=Signal::derive(move || {
                    format!(
                        "{} {}",
                        sider_style.get().unwrap_or_default(),
                        content_style.get().unwrap_or_default(),
                    )
                })
            >

                {children()}
            </Scrollbar>
        </div>
    }
}

fn gen_class(position: LayoutPosition) -> String {
    let mut class = String::from("thaw-layout");
    if position == LayoutPosition::Absolute {
        class.push_str(" thaw-layout--absolute-positioned");
    }
    class
}
