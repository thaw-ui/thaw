use leptos::prelude::window;
use web_sys::DomRect;

#[derive(Clone, PartialEq)]
pub enum FollowerPlacement {
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

impl Copy for FollowerPlacement {}

impl FollowerPlacement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
            Self::Left => "left",
            Self::Right => "right",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",
            Self::LeftStart => "left-start",
            Self::LeftEnd => "left-end",
            Self::RightStart => "right-start",
            Self::RightEnd => "right-end",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }

    pub fn transform_origin(&self) -> &'static str {
        match self {
            Self::Top => "bottom center",
            Self::Bottom => "top center",
            Self::Left => "center right",
            Self::Right => "center left",
            Self::TopStart => "bottom left",
            Self::TopEnd => "bottom right",
            Self::LeftStart => "top right",
            Self::LeftEnd => "bottom right",
            Self::RightStart => "top left",
            Self::RightEnd => "bottom left",
            Self::BottomStart => "top left",
            Self::BottomEnd => "top right",
        }
    }
}

pub struct FollowerPlacementOffset {
    pub top: f64,
    pub left: f64,
    pub transform: String,
    pub placement: FollowerPlacement,
    pub max_height: Option<f64>,
}

pub fn get_follower_placement_offset(
    placement: FollowerPlacement,
    target_rect: DomRect,
    follower_rect: DomRect,
    content_rect: DomRect,
) -> Option<FollowerPlacementOffset> {
    let (left, placement, top, transform, max_height) = match placement {
        FollowerPlacement::Top | FollowerPlacement::TopStart | FollowerPlacement::TopEnd => {
            let Some(window_inner_height) = window_inner_height() else {
                return None;
            };
            let content_height = content_rect.height();
            let target_top = target_rect.top();
            let target_bottom = target_rect.bottom();
            let top = target_top - content_height;
            let (top, new_placement, max_height) =
                if top < 0.0 && target_bottom + content_height <= window_inner_height {
                    let new_placement = if placement == FollowerPlacement::Top {
                        FollowerPlacement::Bottom
                    } else if placement == FollowerPlacement::TopStart {
                        FollowerPlacement::BottomStart
                    } else if placement == FollowerPlacement::TopEnd {
                        FollowerPlacement::BottomEnd
                    } else {
                        unreachable!()
                    };
                    (target_bottom, new_placement, window_inner_height - target_bottom)
                } else {
                    (top, placement, window_inner_height - target_top)
                };

            if placement == FollowerPlacement::Top {
                let left = target_rect.left() + target_rect.width() / 2.0;
                let transform = String::from("translateX(-50%)");
                (left, new_placement, top, transform, Some(max_height))
            } else if placement == FollowerPlacement::TopStart {
                let left = target_rect.left();
                let transform = String::new();
                (left, new_placement, top, transform, Some(max_height))
            } else if placement == FollowerPlacement::TopEnd {
                let left = target_rect.right();
                let transform = String::from("translateX(-100%)");
                (left, new_placement, top, transform, Some(max_height))
            } else {
                unreachable!()
            }
        }
        FollowerPlacement::Bottom
        | FollowerPlacement::BottomStart
        | FollowerPlacement::BottomEnd => {
            let Some(window_inner_height) = window_inner_height() else {
                return None;
            };
            let content_height = content_rect.height();
            let target_top = target_rect.top();
            let target_bottom = target_rect.bottom();
            let top = target_bottom;
            let (top, new_placement, max_height) = if top + content_height > window_inner_height
                && target_top - content_height >= 0.0
            {
                let new_placement = if placement == FollowerPlacement::Bottom {
                    FollowerPlacement::Top
                } else if placement == FollowerPlacement::BottomStart {
                    FollowerPlacement::TopStart
                } else if placement == FollowerPlacement::BottomEnd {
                    FollowerPlacement::TopEnd
                } else {
                    unreachable!()
                };
                (target_top - content_height, new_placement, window_inner_height - target_top)
            } else {
                (top, placement, window_inner_height - target_bottom)
            };
            if placement == FollowerPlacement::Bottom {
                let left = target_rect.left() + target_rect.width() / 2.0;
                let transform = String::from("translateX(-50%)");
                (left, new_placement, top, transform, Some(max_height))
            } else if placement == FollowerPlacement::BottomStart {
                let left = target_rect.left();
                let transform = String::new();
                (left, new_placement, top, transform, Some(max_height))
            } else if placement == FollowerPlacement::BottomEnd {
                let left = target_rect.right();
                let transform = String::from("translateX(-100%)");
                (left, new_placement, top, transform, Some(max_height))
            } else {
                unreachable!()
            }
        }
        FollowerPlacement::Left | FollowerPlacement::LeftStart | FollowerPlacement::LeftEnd => {
            let Some(window_inner_width) = window_inner_width() else {
                return None;
            };
            let content_width = content_rect.width();
            let target_left = target_rect.left();
            let target_right = target_rect.right();
            let left = target_left - content_width;

            let (left, new_placement) =
                if left < 0.0 && target_right + content_width <= window_inner_width {
                    let new_placement = if placement == FollowerPlacement::Left {
                        FollowerPlacement::Right
                    } else if placement == FollowerPlacement::LeftStart {
                        FollowerPlacement::RightStart
                    } else if placement == FollowerPlacement::LeftEnd {
                        FollowerPlacement::RightEnd
                    } else {
                        unreachable!()
                    };
                    (target_right, new_placement)
                } else {
                    (left, placement)
                };
            if placement == FollowerPlacement::Left {
                let top = target_rect.top() + target_rect.height() / 2.0;
                let transform = String::from("translateY(-50%)");
                (left, new_placement, top, transform, None)
            } else if placement == FollowerPlacement::LeftStart {
                let top = target_rect.top();
                let transform = String::new();
                (left, new_placement, top, transform, None)
            } else if placement == FollowerPlacement::LeftEnd {
                let top = target_rect.bottom();
                let transform = String::from("translateY(-100%)");
                (left, new_placement, top, transform, None)
            } else {
                unreachable!()
            }
        }
        FollowerPlacement::Right | FollowerPlacement::RightStart | FollowerPlacement::RightEnd => {
            let Some(window_inner_width) = window_inner_width() else {
                return None;
            };

            let content_width = content_rect.width();
            let target_left = target_rect.left();
            let target_right = target_rect.right();
            let left = target_right;
            let (left, new_placement) = if left + content_width > window_inner_width
                && target_left - content_width >= 0.0
            {
                let new_placement = if placement == FollowerPlacement::Right {
                    FollowerPlacement::Left
                } else if placement == FollowerPlacement::RightStart {
                    FollowerPlacement::LeftStart
                } else if placement == FollowerPlacement::RightEnd {
                    FollowerPlacement::LeftEnd
                } else {
                    unreachable!()
                };
                (target_left - content_width, new_placement)
            } else {
                (left, placement)
            };

            if placement == FollowerPlacement::Right {
                let top = target_rect.top() + target_rect.height() / 2.0;
                let transform = String::from("translateY(-50%)");
                (left, new_placement, top, transform, None)
            } else if placement == FollowerPlacement::RightStart {
                let top = target_rect.top();
                let transform = String::new();
                (left, new_placement, top, transform, None)
            } else if placement == FollowerPlacement::RightEnd {
                let top = target_rect.bottom();
                let transform = String::from("translateY(-100%)");
                (left, new_placement, top, transform, None)
            } else {
                unreachable!()
            }
        }
    };

    Some(FollowerPlacementOffset {
        top: top - follower_rect.top(),
        left: left - follower_rect.left(),
        placement,
        transform,
        max_height,
    })
}

fn window_inner_width() -> Option<f64> {
    let Ok(inner_width) = window().inner_width() else {
        return None;
    };
    let Some(inner_width) = inner_width.as_f64() else {
        return None;
    };
    Some(inner_width)
}

fn window_inner_height() -> Option<f64> {
    let Ok(inner_height) = window().inner_height() else {
        return None;
    };
    let Some(inner_height) = inner_height.as_f64() else {
        return None;
    };
    Some(inner_height)
}
