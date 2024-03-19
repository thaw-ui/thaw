use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn ButtonGroup(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] vertical: bool,
    children: Children,
) -> impl IntoView {
    mount_style("button-group", include_str!("./button-group.css"));
    view! {
        <div class=class_list!["thaw-button-group", class.map(| c | move || c.get())] class=("thaw-button-group--vertical", vertical)>
            {children()}
        </div>
    }
}
