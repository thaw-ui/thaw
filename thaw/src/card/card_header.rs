use leptos::prelude::*;
use thaw_components::OptionComp;
use thaw_utils::{class_list, mount_style};

#[component]
pub fn CardHeader(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional)] card_header_description: Option<CardHeaderDescription>,
    #[prop(optional)] card_header_action: Option<CardHeaderAction>,
    children: Children,
) -> impl IntoView {
    mount_style("card-header", include_str!("./card-header.css"));
    view! {
        <div class=class_list!["thaw-card-header", class]>
            <div class="thaw-card-header__header">
                {children()}
            </div>
            <OptionComp value=card_header_description let:description>
                <div class="thaw-card-header__description">
                    {(description.children)()}
                </div>
            </OptionComp>
            <OptionComp value=card_header_action let:action>
                <div class="thaw-card-header__action">
                    {(action.children)()}
                </div>
            </OptionComp>
        </div>
    }
}

#[slot]
pub struct CardHeaderDescription {
    children: Children,
}

#[slot]
pub struct CardHeaderAction {
    children: Children,
}
