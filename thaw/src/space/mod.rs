use leptos::prelude::*;
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal, OptionalProp};

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
    #[prop(optional, into)] align: OptionalMaybeSignal<SpaceAlign>,
    #[prop(optional, into)] justify: OptionalMaybeSignal<SpaceJustify>,
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
            style:align-items=move || align.get().map(|a| a.as_str())
            style:justify-content=move || justify.get().map(|j| j.as_str())
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
