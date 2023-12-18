#[cfg(not(feature = "ssr"))]
use crate::utils::dyn_classes;
use crate::utils::{mount_style, ssr_class};
use leptos::*;

#[derive(Default)]
pub enum SpaceGap {
    Small,
    #[default]
    Medium,
    Large,
    Size(u16),
    /// width and height
    WH(u16, u16),
}

#[component]
pub fn Space(
    #[prop(optional)] gap: SpaceGap,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("space", include_str!("./space.css"));
    let gap = match gap {
        SpaceGap::Small => "4px 8px".into(),
        SpaceGap::Medium => "8px 12px".into(),
        SpaceGap::Large => "12px 16px".into(),
        SpaceGap::Size(size) => format!("{size}px {size}px"),
        SpaceGap::WH(width, height) => format!("{width}px {height}px"),
    };

    let ssr_class = ssr_class(&class);
    view! {
        <div
            class=ssr_class
            use:dyn_classes=class
            class="thaw-space"
            style:gap=gap
            style:flex-direction=if vertical { "column" } else { "row" }
        >

            {children()
                .nodes
                .into_iter()
                .map(|node| {
                    view! { <div class="thaw-space__item">{node}</div> }
                })
                .collect::<Vec<_>>()}

        </div>
    }
}


