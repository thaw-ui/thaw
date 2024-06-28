use leptos::*;
use thaw_components::OptionComp;

#[component]
pub fn DrawerHeaderTitle(
    #[prop(optional)] drawer_header_title_action: Option<DrawerHeaderTitleAction>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="thaw-drawer-header-title">
            <h2 class="thaw-drawer-header-title__heading">
                {children()}
            </h2>
            <OptionComp value=drawer_header_title_action let:action>
                <div class="thaw-drawer-header-title__action">
                    {(action.children)()}
                </div>
            </OptionComp>
        </div>
    }
}

#[slot]
pub struct DrawerHeaderTitleAction {
    children: Children,
}
