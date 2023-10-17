use crate::{
    components::*,
    icon::*,
    utils::{mount_style::mount_style, StoredMaybeSignal},
};
use leptos::*;

#[component]
pub fn NavBar(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional, into)] left_arrow: MaybeSignal<bool>,
    #[prop(optional, into)] left_text: MaybeSignal<String>,
    #[prop(optional, into)] on_click_left: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] right_text: MaybeSignal<String>,
    #[prop(optional, into)] on_click_right: Option<Callback<ev::MouseEvent>>,
) -> impl IntoView {
    mount_style("nav-bar", include_str!("./nav-bar.css"));
    let title: StoredMaybeSignal<_> = title.into();
    let left_text: StoredMaybeSignal<_> = left_text.into();
    let right_text: StoredMaybeSignal<_> = right_text.into();

    let on_click_left = move |ev| {
        if let Some(click_left) = on_click_left.as_ref() {
            click_left.call(ev);
        }
    };

    let on_click_right = move |ev| {
        if let Some(click_right) = on_click_right.as_ref() {
            click_right.call(ev);
        }
    };

    view! {
        <div class="melt-nav-bar">
            <If cond=MaybeSignal::derive(move || left_arrow.get() || !left_text.get().is_empty())>
                <Then slot>
                    <div class="melt-nav-bar__left" on:click=on_click_left>
                        <If cond=left_arrow>
                            <Then slot>
                                <Icon icon=Icon::from(AiIcon::AiLeftOutlined)/>
                            </Then>
                        </If>
                        {move || left_text.get()}
                    </div>
                </Then>
            </If>
            <div class="melt-nav-bar__center">{move || title.get()}</div>
            <If cond=MaybeSignal::derive(move || !right_text.get().is_empty())>
                <Then slot>
                    <div class="melt-nav-bar__right" on:click=on_click_right>
                        {move || right_text.get()}
                    </div>
                </Then>
            </If>
        </div>
    }
}
