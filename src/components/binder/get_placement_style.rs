use leptos::window;
use web_sys::DomRect;

#[derive(Clone)]
pub enum FollowerPlacement {
    Top,
    // Bottom,
    // Left,
    // Right,
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

fn window_inner_height() -> Option<f64> {
    let Ok(inner_height) = window().inner_height() else {
        return None;
    };
    let Some(inner_height) = inner_height.as_f64() else {
        return None;
    };
    Some(inner_height)
}
