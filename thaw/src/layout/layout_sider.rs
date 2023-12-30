use crate::utils::{class_list::class_list, mount_style};
use leptos::*;

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("layout-sider", include_str!("./layout-sider.css"));
    view! {
        <div class=class_list!["thaw-layout-sider", move || class.get()] style=move || style.get()>
            {children()}
        </div>
    }
}
