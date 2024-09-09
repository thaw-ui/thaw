use leptos::*;
use thaw_utils::{class_list, OptionalMaybeSignal, OptionalProp};

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

#[component]
pub fn Flex(
    #[prop(optional)] gap: FlexGap,
    #[prop(optional)] vertical: bool,
    #[prop(optional, into)] align: OptionalMaybeSignal<FlexAlign>,
    #[prop(optional, into)] justify: OptionalMaybeSignal<FlexJustify>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] style: OptionalProp<MaybeSignal<String>>,
    children: Children,
) -> impl IntoView {
    let style = style.into_option();
    let style = Memo::new(move |_| {
        let mut s = String::from("display: flex;");
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
        if let Some(justify) = justify.get() {
            s.push_str(&format!("justify-content: {};", justify.as_str()));
        }
        if let Some(style) = style.as_ref() {
            s.push_str(&style.get());
        }
        s
    });

    view! {
        <div
            class=class_list!["thaw-flex", class.map(| c | move || c.get())]
            style=move || style.get()
        >
            {children()}
        </div>
    }
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
