use crate::utils::mount_style::mount_style;
use leptos::*;

#[component]
pub fn LayoutSider(
    #[prop(optional, into)] style: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("layout-sider", include_str!("./layout-sider.css"));
    view! {
        <div class="melt-layout-sider" style=move || style.get()>
            { children() }
        </div>
    }
}
