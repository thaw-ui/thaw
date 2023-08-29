use crate::utils::mount_style::mount_style;
use leptos::*;
use stylers::style_sheet_str;

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
pub fn Space(#[prop(optional)] gap: SpaceGap, children: Children) -> impl IntoView {
    let class_name = mount_style("space", || style_sheet_str!("./src/space/space.css"));
    let gap = match gap {
        SpaceGap::SMALL => "gap: 4px 8px".into(),
        SpaceGap::MEDIUM => "gap: 8px 12px".into(),
        SpaceGap::LARGE => "gap: 12px 16px".into(),
        SpaceGap::NUMBER(size) => format!("gap: {size}px {size}px"),
        SpaceGap::TUPLE(x, y) => format!("gap: {x}px {y}px"),
    };

    view! {
         class=class_name,
        <div class="melt-space" style=format!("{gap};")>
            {
                children().nodes.into_iter().map(|node| {
                    view! {
                         class=class_name,
                        <div class="melt-space__item">
                            {node}
                        </div>
                    }
                }).collect::<Vec<_>>()
            }
        </div>
    }
}
