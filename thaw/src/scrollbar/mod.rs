use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[component]
pub fn Scrollbar(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: Option<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    mount_style("scrollbar", include_str!("./scrollbar.css"));

    view! {
        <div
            class=class_list!["thaw-scrollbar",  class.map(| c | move || c.get())]
            style=move || style.as_ref().map(|s| s.get())
        >
            <div class="thaw-scrollbar__container">
                <div class="thaw-scrollbar__content">
                    {children()}
                </div>
            </div>
            <div class="thaw-scrollbar__track--vertical">
                <div class="thaw-scrollabr__thumb"></div>
            </div>
            <div class="thaw-scrollbar__track--horizontal">
                <div class="thaw-scrollabr__thumb"></div>
            </div>
        </div>
    }
}
