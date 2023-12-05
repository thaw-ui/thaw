use crate::utils::mount_style;
use leptos::*;

#[component]
pub fn ButtonGroup(#[prop(optional)] vertical: bool, children: Children) -> impl IntoView {
    mount_style("button-group", include_str!("./button-group.css"));
    view! {
        <div class="thaw-button-group" class=("thaw-button-group--vertical", vertical)>
            {children()}
        </div>
    }
}
