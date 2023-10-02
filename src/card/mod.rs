use std::rc::Rc;

use crate::{components::*, utils::mount_style::mount_style};
use leptos::*;
use stylers::style_sheet_str;

#[slot]
pub struct CardHeader {
    children: ChildrenFn,
}

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
    #[prop(optional, into)] title: MaybeSignal<&'static str>,
    #[prop(optional)] card_header: Option<CardHeader>,
    #[prop(optional)] card_header_extra: Option<CardHeaderExtra>,
    children: Children,
    #[prop(optional)] card_footer: Option<CardFooter>,
) -> impl IntoView {
    let class_name = mount_style("card", || style_sheet_str!("./src/card/card.css"));

    let is_header = card_header.is_some();
    let header = card_header.map_or(None, |v| Some(Rc::new(v)));
    let header_extra = card_header_extra.map_or(None, |v| Some(Rc::new(v)));
    // let footer = card_footer.map_or(None, |v| Some(Rc::new(v)));

    view! {
         class=class_name,
        <div class="melt-card">
                <If cond=MaybeSignal::derive( move || is_header || !title.get().is_empty())  >
                    <Then slot>
                        <div class="melt-card__header">
                            <div class="melt-card__header-content">
                                <OptionComp value=header.clone() let:header>
                                    <Fallback slot>
                                        { title.get() }
                                    </Fallback>
                                    { (header.children)() }
                                </OptionComp>
                            </div>
                            <OptionComp value=header_extra.clone() let:header_extra>
                                <div class="melt-card__header-extra">
                                    { (header_extra.children)() }
                                </div>
                            </OptionComp>
                        </div>
                    </Then>
                </If>
            <div class="melt-card__content">
                { children() }
            </div>
            <OptionComp value=card_footer let:footer>
                <If cond=footer.if_ >
                    <Then slot>
                        <div class="melt-card__footer">
                            { (footer.children)() }
                        </div>
                    </Then>
                </If>
            </OptionComp>
         </div>
    }
}
