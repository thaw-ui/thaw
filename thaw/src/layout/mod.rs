mod layout_header;
mod layout_sider;

use crate::utils::{class_list::class_list, mount_style, OptionalProp};
pub use layout_header::*;
pub use layout_sider::*;
use leptos::*;

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
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] position: LayoutPosition,
    #[prop(optional, into)] has_sider: MaybeSignal<bool>,
    children: Children,
) -> impl IntoView {
    mount_style("layout", include_str!("./layout.css"));

    let style = create_memo(move |_| {
        let mut new_style = style.as_ref().map(|s| s.get()).unwrap_or_default();
        if has_sider.get() {
            new_style
                .push_str("display: flex; flex-wrap: nowrap; flex-direction: row; width: 100%;");

            Some(new_style)
        } else if style.is_some() {
            Some(new_style)
        } else {
            None
        }
    });
    view! {
        <div
            class=class_list![gen_class(position), class.map(|c| move || c.get())]
            style=move || style.get()
        >
            {children()}
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
