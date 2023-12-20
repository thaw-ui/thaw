use leptos::window;
use web_sys::DomRect;

#[derive(Clone)]
pub enum FollowerPlacement {
    Top,
    Bottom,
    Left,
    Right,
    // TopStart,
    // TopEnd,
    // LeftStart,
    // LeftEnd,
    // RightStart,
    // RightEnd,
    BottomStart,
    // BottomEnd,
}

impl Copy for FollowerPlacement {}

pub struct FollowerPlacementOffset {
    pub top: f64,
    pub left: f64,
    pub transform: String,
}

pub fn get_follower_placement_offset(
    placement: FollowerPlacement,
    target_rect: DomRect,
    follower_rect: DomRect,
) -> Option<FollowerPlacementOffset> {
    match placement {
        FollowerPlacement::Top => {
            let left = target_rect.x() + target_rect.width() / 2.0;
            let top = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y - follower_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top < 0.0 && target_y + target_height + follower_height <= inner_height {
                    target_y + target_height
                } else {
                    top
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-50%)"),
            })
        }
        FollowerPlacement::Bottom => {
            let left = target_rect.x() + target_rect.width() / 2.0;
            let top = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y + target_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top + follower_height > inner_height && target_y - follower_height >= 0.0 {
                    target_y - follower_height
                } else {
                    top
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-50%)"),
            })
        }
        FollowerPlacement::Left => {
            let top = target_rect.x() + target_rect.height() / 2.0;
            let left = {
                let follower_width = follower_rect.width();
                let target_y = target_rect.y();
                let target_width = target_rect.width();
                let left = target_y - follower_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left < 0.0 && target_y + target_width + follower_width > inner_width {
                    target_y + follower_width
                } else {
                    left
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-50%)"),
            })
        }
        FollowerPlacement::Right => {
            let top = target_rect.x() + target_rect.height() / 2.0;
            let left = {
                let follower_width = follower_rect.width();
                let target_y = target_rect.y();
                let target_width = target_rect.width();
                let left = target_y + target_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left + follower_width > inner_width && target_y - follower_width >= 0.0 {
                    target_y - follower_width
                } else {
                    left
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-50%)"),
            })
        }
        FollowerPlacement::BottomStart => {
            let left = target_rect.x();
            let top = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y + target_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top + follower_height > inner_height && target_y - follower_height >= 0.0 {
                    target_y - follower_height
                } else {
                    top
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::new(),
            })
        }
    }
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
