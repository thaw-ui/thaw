use crate::ConfigInjection;
use leptos::{
    ev::{self, on},
    html,
    leptos_dom::helpers::TimeoutHandle,
    prelude::*,
    tachys::html::class::class as tachys_class,
};
use std::time::Duration;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement};
use thaw_utils::{class_list, mount_style};

#[component]
pub fn Tooltip<T>(
    /// The text of the tooltip.
    #[prop(optional, into)]
    content: Option<Signal<String>>,
    /// Configure the position of the tooltip.
    #[prop(optional)]
    position: TooltipPosition,
    /// The tooltip's visual appearance.
    #[prop(optional, into)]
    appearance: Signal<TooltipAppearance>,
    children: TypedChildren<T>,
) -> impl IntoView
where
    T: AddAnyAttr + IntoView + Send + 'static,
{
    mount_style("tooltip", include_str!("./tooltip.css"));
    let config_provider = ConfigInjection::expect_context();

    let content_ref = NodeRef::<html::Div>::new();
    let is_show_content = RwSignal::new(false);
    let content_handle = StoredValue::new(None::<TimeoutHandle>);

    let on_mouse_enter = move |_| {
        content_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
        is_show_content.set(true);
    };
    let on_mouse_leave = move |_| {
        content_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
            *handle = set_timeout_with_handle(
                move || {
                    is_show_content.set(false);
                },
                Duration::from_millis(100),
            )
            .ok();
        });
    };

    Owner::on_cleanup(move || {
        content_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.clear();
            }
        });
    });

    view! {
        <Binder>
            {children
                .into_inner()()
                .into_inner()
                .add_any_attr(tachys_class(("thaw-tooltip", true)))
                .add_any_attr(on(ev::mouseenter, on_mouse_enter))
                .add_any_attr(on(ev::mouseleave, on_mouse_leave))}
            <Follower slot show=is_show_content placement=position>
                <CSSTransition
                    node_ref=content_ref
                    name="tooltip-transition"
                    appear=is_show_content.get_untracked()
                    show=is_show_content
                    let:display
                >
                    <div
                        class=class_list![
                            "thaw-config-provider thaw-tooltip-content",
                            move || format!("thaw-tooltip-content--{}", appearance.get().as_str())
                        ]
                        data-thaw-id=config_provider.id()
                        style=move || display.get().unwrap_or_default()
                        role="tooltip"
                        node_ref=content_ref
                        on:mouseenter=on_mouse_enter
                        on:mouseleave=on_mouse_leave
                    >
                        {move || { content.as_ref().map(|c| c.get()).unwrap_or_default() }}
                        <div class="thaw-tooltip-content__angle"></div>
                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Clone, Default)]
pub enum TooltipAppearance {
    #[default]
    Normal,
    Inverted,
}

impl TooltipAppearance {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Inverted => "inverted",
        }
    }
}

#[derive(Default)]
pub enum TooltipPosition {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
    TopStart,
    TopEnd,
    LeftStart,
    LeftEnd,
    RightStart,
    RightEnd,
    BottomStart,
    BottomEnd,
}

impl From<TooltipPosition> for FollowerPlacement {
    fn from(value: TooltipPosition) -> Self {
        match value {
            TooltipPosition::Top => Self::Top,
            TooltipPosition::Bottom => Self::Bottom,
            TooltipPosition::Left => Self::Left,
            TooltipPosition::Right => Self::Right,
            TooltipPosition::TopStart => Self::TopStart,
            TooltipPosition::TopEnd => Self::TopEnd,
            TooltipPosition::LeftStart => Self::LeftStart,
            TooltipPosition::LeftEnd => Self::LeftEnd,
            TooltipPosition::RightStart => Self::RightStart,
            TooltipPosition::RightEnd => Self::RightEnd,
            TooltipPosition::BottomStart => Self::BottomStart,
            TooltipPosition::BottomEnd => Self::BottomEnd,
        }
    }
}
