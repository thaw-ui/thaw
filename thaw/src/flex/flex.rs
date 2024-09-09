use leptos::prelude::*;
use thaw_utils::class_list;

#[component]
pub fn Flex(
    #[prop(optional, into)] class: MaybeProp<String>,
    #[prop(optional, into)] style: MaybeProp<String>,
    /// Flex's gap.
    #[prop(optional)]
    gap: FlexGap,
    /// Whether to lay out vertically.
    #[prop(optional)]
    vertical: bool,
    /// Whether it's display is `inline-flex`.
    #[prop(optional, into)]
    inline: MaybeSignal<bool>,
    /// Vertical arrangement.
    #[prop(optional, into)]
    align: MaybeProp<FlexAlign>,
    /// Horizontal arrangement.
    #[prop(optional, into)]
    justify: MaybeProp<FlexJustify>,
    children: Children,
) -> impl IntoView {
    let style = Memo::new(move |_| {
        let mut s = String::new();
        let display = if inline.get() {
            "display: inline-flex;"
        } else {
            "display: flex;"
        };
        s.push_str(display);
        let direction = if vertical {
            "flex-direction: column;"
        } else {
            "flex-direction: row;"
        };
        let gap = match gap {
            FlexGap::Small => "gap: 4px 8px;",
            FlexGap::Medium => "gap: 8px 12px;",
            FlexGap::Large => "gap: 12px 16px;",
            FlexGap::Size(size) => &format!("gap: {size}px {size}px;"),
            FlexGap::WH(width, height) => &format!("gap: {width}px {height}px;"),
        };
        s.push_str(direction);
        s.push_str(gap);
        if let Some(align) = align.get() {
            s.push_str(&format!("align-items: {};", align.as_str()));
        }
        if let Some(justify) = justify.get() {
            s.push_str(&format!("justify-content: {};", justify.as_str()));
        }
        style.with(|style| {
            if let Some(style) = style.as_ref() {
                s.push_str(style);
            }
        });

        s
    });

    view! {
        <div class=class_list!["thaw-flex", class] style=move || style.get()>
            {children()}
        </div>
    }
}

#[derive(Default)]
pub enum FlexGap {
    Small,
    #[default]
    Medium,
    Large,
    Size(u16),
    /// width and height
    WH(u16, u16),
}

#[derive(Clone)]
pub enum FlexAlign {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    Baseline,
    Stretch,
}

impl FlexAlign {
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
pub enum FlexJustify {
    FlexStart,
    FlexEnd,
    Start,
    End,
    Center,
    SpaceAround,
    SpaceBetween,
    SpaceEvenly,
}

impl FlexJustify {
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
