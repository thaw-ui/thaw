use crate::utils::mount_style::mount_style;
use leptos::*;

#[derive(Default)]
pub enum SpaceGap {
    SMALL,
    #[default]
    MEDIUM,
    LARGE,
    NUMBER(u16),
    TUPLE(u16, u16),
}

#[component]
pub fn Space(
    #[prop(optional)] gap: SpaceGap,
    #[prop(optional)] vertical: bool,
    children: Children,
) -> impl IntoView {
    mount_style("space", include_str!("./space.css"));
    let gap = match gap {
        SpaceGap::SMALL => "4px 8px".into(),
        SpaceGap::MEDIUM => "8px 12px".into(),
        SpaceGap::LARGE => "12px 16px".into(),
        SpaceGap::NUMBER(size) => format!("{size}px {size}px"),
        SpaceGap::TUPLE(x, y) => format!("{x}px {y}px"),
    };

    view! {
        <div class="melt-space" style:gap={gap} style:flex-direction=if vertical { "column" } else { "row" }>
            {
                children().nodes.into_iter().map(|node| {
                    view! {
                        <div class="melt-space__item">
                            {node}
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
