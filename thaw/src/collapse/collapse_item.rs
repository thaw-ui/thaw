use crate::{utils::class_list::class_list, Icon};
use icondata::AiIcon;
use leptos::*;

#[component]
pub fn CollapseItem(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(into)] title: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let is_show_content = create_rw_signal(false);

    view! {
        <div class=class_list![
            "thaw-collapse-item", ("thaw-collapse-item--active", move || is_show_content.get()),
            move || class.get()
        ]>
            <div
                class="thaw-collapse-item__header"
                on:click=move |_| is_show_content.update(|show| *show = !*show)
            >
                <Icon icon=Icon::from(AiIcon::AiRightOutlined) class="thaw-collapse-item-arrow"/>
                {move || title.get()}
            </div>
            <div
                class="thaw-collapse-item__content"
                style=move || (!is_show_content.get()).then_some("display: none;")
            >
                {children()}
            </div>
        </div>
    }
}
