use super::use_collapse;
use crate::{
    components::CSSTransition,
    utils::{class_list::class_list, OptionalProp, StoredMaybeSignal},
    Icon,
};
use leptos::*;

#[component]
pub fn CollapseItem(
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] key: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    let collapse = use_collapse();
    let key: StoredMaybeSignal<_> = key.into();
    let content_ref = create_node_ref::<html::Div>();

    let is_show_content = create_memo(move |_| {
        collapse
            .value
            .with(|keys| key.with(|key| keys.contains(key)))
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
            class.map(| c | move || c.get())
        ]>
            <div class="thaw-collapse-item__header" on:click=on_click>
                <Icon icon=icondata_ai::AiRightOutlined class="thaw-collapse-item-arrow"/>
                {move || title.get()}
            </div>
            <CSSTransition
                node_ref=content_ref
                show=is_show_content
                name="thaw-collapse-item"
                let:display
            >
                <div
                    class="thaw-collapse-item__content"
                    ref=content_ref
                    style=move || display.get()
                >
                    {children()}
                </div>
            </CSSTransition>
        </div>
    }
}
