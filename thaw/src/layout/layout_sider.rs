use crate::Scrollbar;
use leptos::prelude::*;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] content_class: MaybeProp<String>,
    #[prop(optional, into)] content_style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    mount_style("layout-sider", include_str!("./layout-sider.css"));
    view! {
        <div class=class_list!["thaw-layout-sider", class]>
            <Scrollbar content_class content_style>
                {children()}
            </Scrollbar>
        </div>
    }
}
