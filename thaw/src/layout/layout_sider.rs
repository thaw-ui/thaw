use crate::utils::{class_list::class_list, mount_style, OptionalProp};
use leptos::*;

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("layout-sider", include_str!("./layout-sider.css"));
    view! {
        <div class=class_list!["thaw-layout-sider", class.map(|c| move || c.get())] style=style.map(|s| move || s.get())>
            {children()}
        </div>
    }
}
