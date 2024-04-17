use leptos::*;
use thaw_components::OptionComp;

#[component]
pub fn AnchorLink(
    #[prop(into)] title: MaybeSignal<String>,
    #[prop(into)] href: MaybeSignal<String>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div class="thaw-anchor-link">
            <a href=move || href.get() class="thaw-anchor-link__title">
                {move || title.get()}
            </a>
            <OptionComp value=children let:children>
                {children()}
            </OptionComp>
        </div>
    }
}
