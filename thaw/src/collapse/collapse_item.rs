use super::use_collapse;
use crate::{
    utils::{class_list::class_list, StoredMaybeSignal},
    Icon,
};
use icondata::AiIcon;
use leptos::*;

#[component]
pub fn CollapseItem(
    #[prop(optional, into)] class: MaybeSignal<String>,
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] key: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let collapse = use_collapse();
    let key: StoredMaybeSignal<_> = key.into();
    let is_show_content = create_memo(move |_| {
        collapse.value.with(|keys| {
            if key.with(|key| keys.contains(key)) {
                true
            } else {
                false
            }
        })
    });

    let on_click = move |_| {
        collapse.value.update(|keys| {
            if collapse.accordion {
                keys.clear();
            }
            let key = key.get_untracked();
            if is_show_content.get_untracked() {
                keys.remove(&key);
            } else {
                keys.insert(key);
            }
        });
    };

    view! {
        <div class=class_list![
            "thaw-collapse-item", ("thaw-collapse-item--active", move || is_show_content.get()),
            move || class.get()
        ]>
            <div class="thaw-collapse-item__header" on:click=on_click>
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
