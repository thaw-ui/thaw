use crate::Scrollbar;
use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/layout/layout-sider.css");

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Addtional classes for the scroll element.
    #[prop(optional, into)]
    content_class: MaybeProp<String>,
    /// Style of scrollable content node.
    #[prop(optional, into)]
    content_style: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("layout-sider", include_str!("./layout-sider.css"));

    view! {
        <div class=class_list!["thaw-layout-sider", class]>
            <Scrollbar content_class content_style>
                {children()}
            </Scrollbar>
        </div>
    }
}
