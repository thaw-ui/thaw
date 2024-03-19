use crate::{components::*, use_theme, Theme};
use leptos::*;
use thaw_utils::{class_list, mount_style, OptionalProp};

#[slot]
pub struct CardHeader {
    children: Children,
}

#[slot]
pub struct CardHeaderExtra {
    children: Children,
}

#[slot]
pub struct CardFooter {
    #[prop(default = leptos::MaybeSignal::Static(true), into)]
    if_: MaybeSignal<bool>,
    children: ChildrenFn,
}

#[component]
pub fn Card(
    #[prop(optional, into)] title: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] card_header: Option<CardHeader>,
    #[prop(optional)] card_header_extra: Option<CardHeaderExtra>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
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

    view! {
        <div
            class=class_list!["thaw-card", class.map(| c | move || c.get())]
            style=move || css_vars.get()
        >

            {if card_header.is_some() || title.is_some() {
                view! {
                    <div class="thaw-card__header">
                        <div class="thaw-card__header-content">

                            {if let Some(header) = card_header {
                                (header.children)().into_view()
                            } else if let Some(title) = title.into_option() {
                                (move || title.get()).into_view()
                            } else {
                                unreachable!()
                            }}

                        </div>
                        <OptionComp value=card_header_extra let:header_extra>
                            <div class="thaw-card__header-extra">{(header_extra.children)()}</div>
                        </OptionComp>
                    </div>
                }
                    .into()
            } else {
                None
            }}

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
