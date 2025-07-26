use leptos::prelude::*;
use thaw_utils::class_list;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/space/space.css");

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
    #[prop(optional, into)] class: MaybeProp<String>,
    /// Space's gap.
    #[prop(optional)]
    gap: SpaceGap,
    /// Whether to lay out vertically.
    #[prop(optional)]
    vertical: bool,
    ///  Vertical arrangement.
    #[prop(optional, into)]
    align: MaybeProp<SpaceAlign>,
    /// Horizontal arrangement.
    #[prop(optional, into)]
    justify: MaybeProp<SpaceJustify>,
    children: ChildrenFragment,
) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("space", include_str!("./space.css"));

    let gap = match gap {
        SpaceGap::Small => "4px 8px".into(),
        SpaceGap::Medium => "8px 12px".into(),
        SpaceGap::Large => "12px 16px".into(),
        SpaceGap::Size(size) => format!("{size}px {size}px"),
        SpaceGap::WH(width, height) => format!("{width}px {height}px"),
    };

    view! {
        <div
            class=class_list!["thaw-space", class]
            style:gap=gap
            style:align-items=move || align.get().map(|a| a.as_str()).unwrap_or_default()
            style:justify-content=move || justify.get().map(|j| j.as_str()).unwrap_or_default()
            style:flex-direction=if vertical { "column" } else { "row" }
        >

            {children()
                .nodes
                .into_iter()
                .map(|node| {
                    view! { <div class="thaw-space__item">{node}</div> }
                })
                .collect_view()}

        </div>
    }
}

#[derive(Clone)]
pub enum SpaceAlign {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl SpaceAlign {
    fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::FlexEnd => "flex-end",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::Baseline => "baseline",
            Self::Stretch => "stretch",
        }
    }
}

#[derive(Clone)]
pub enum SpaceJustify {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

impl SpaceJustify {
    fn as_str(&self) -> &'static str {
        match self {
            Self::FlexStart => "flex-start",
            Self::FlexEnd => "flex-end",
            Self::Start => "start",
            Self::End => "end",
            Self::Center => "center",
            Self::SpaceAround => "space-around",
            Self::SpaceBetween => "space-between",
            Self::SpaceEvenly => "space-evenly",
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn main() {
        use super::Space;
        use leptos::prelude::*;

        view! {
            <Space>
                <p></p>
                <p></p>
            </Space>
        };
    }
}
