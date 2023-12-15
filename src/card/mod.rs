#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::{
    components::*,
    use_theme,
    utils::{mount_style, ssr_class},
    Theme,
};
use leptos::*;

#[derive(Clone)]
#[slot]
pub struct CardHeader {
    children: ChildrenFn,
}

#[derive(Clone)]
#[slot]
pub struct CardHeaderExtra {
    children: ChildrenFn,
}

#[slot]
pub struct CardFooter {
    #[prop(default = leptos::MaybeSignal::Static(true), into)]
    if_: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[component]
pub fn Card(
    #[prop(optional, into)] title: MaybeSignal<String>,
    #[prop(optional)] card_header: Option<CardHeader>,
    #[prop(optional)] card_header_extra: Option<CardHeaderExtra>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
    #[prop(optional)] card_footer: Option<CardFooter>,
) -> impl IntoView {
    mount_style("card", include_str!("./card.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.common.background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.common.border_color
            ));
        });
        css_vars
    });

    let title = store_value(title);
    let is_header = card_header.is_some();
    let is_header = MaybeSignal::derive(move || is_header || !title.get_value().get().is_empty());
    let header = store_value(card_header);
    let header_extra = store_value(card_header_extra);

    let ssr_class = ssr_class(&class);
    view! {
        <div class=ssr_class use:dyn_classes=class class="thaw-card" style=move || css_vars.get()>
            <If cond=is_header>
                <Then slot>
                    <div class="thaw-card__header">
                        <div class="thaw-card__header-content">
                            <OptionComp value=header.get_value() let:header>
                                <Fallback slot>{move || title.get_value().get()}</Fallback>
                                {(header.children)()}
                            </OptionComp>
                        </div>
                        <OptionComp value=header_extra.get_value() let:header_extra>
                            <div class="thaw-card__header-extra">{(header_extra.children)()}</div>
                        </OptionComp>
                    </div>
                </Then>
            </If>
            <div class="thaw-card__content">{children()}</div>
            <OptionComp value=card_footer let:footer>
                <If cond=footer.if_>
                    <Then slot>
                        <div class="thaw-card__footer">{(footer.children)()}</div>
                    </Then>
                </If>
            </OptionComp>
        </div>
    }
}

