use leptos::prelude::*;
use thaw_utils::mount_style;

#[component]
pub fn CardFooter(children: Children) -> impl IntoView {
    mount_style("card-footer", include_str!("./card-footer.css"));
    view! {
        <div class="thaw-card-footer">
            {children()}
        </div>
    }
}
