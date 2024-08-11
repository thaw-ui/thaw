use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::class_list;

#[component]
pub fn DrawerHeaderTitle(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] drawer_header_title_action: Option<DrawerHeaderTitleAction>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class=class_list!["thaw-drawer-header-title", class]>
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
