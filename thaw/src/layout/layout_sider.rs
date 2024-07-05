use crate::Scrollbar;
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] content_class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] content_style: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("layout-sider", include_str!("./layout-sider.css"));
    view! {
        <div
            class=class_list!["thaw-layout-sider", class.map(| c | move || c.get())]
        >
            <Scrollbar content_class content_style>
                {children()}
            </Scrollbar>
        </div>
    }
}
