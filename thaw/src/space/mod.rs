use crate::utils::{class_list::class_list, mount_style, OptionalProp};
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
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
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

    view! {
        <div
            class=class_list!["thaw-space", class.map(| c | move || c.get())]
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
