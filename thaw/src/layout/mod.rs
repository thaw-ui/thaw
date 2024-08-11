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
    #[prop(optional, into)] content_class: MaybeProp<String>,
    #[prop(optional, into)] content_style: MaybeProp<String>,
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
