use leptos::window;
use web_sys::DomRect;

#[derive(Clone)]
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
}

pub struct FollowerPlacementOffset {
    pub top: f64,
    pub left: f64,
    pub transform: String,
    pub placement: FollowerPlacement,
}

pub fn get_follower_placement_offset(
    placement: FollowerPlacement,
    target_rect: DomRect,
    follower_rect: DomRect,
) -> Option<FollowerPlacementOffset> {
    match placement {
        FollowerPlacement::Top => {
            let left = target_rect.x() + target_rect.width() / 2.0;
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y - follower_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top < 0.0 && target_y + target_height + follower_height <= inner_height {
                    (target_y + target_height, FollowerPlacement::Bottom)
                } else {
                    (top, FollowerPlacement::Top)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-50%)"),
                placement,
            })
        }
        FollowerPlacement::TopStart => {
            let left = target_rect.x();
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y - follower_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top < 0.0 && target_y + target_height + follower_height <= inner_height {
                    (target_y + target_height, FollowerPlacement::BottomStart)
                } else {
                    (top, FollowerPlacement::TopStart)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::new(),
                placement,
            })
        }
        FollowerPlacement::TopEnd => {
            let left = target_rect.x() + target_rect.width();
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y - follower_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top < 0.0 && target_y + target_height + follower_height <= inner_height {
                    (target_y + target_height, FollowerPlacement::BottomEnd)
                } else {
                    (top, FollowerPlacement::TopEnd)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-100%)"),
                placement,
            })
        }
        FollowerPlacement::Left => {
            let top = target_rect.y() + target_rect.height() / 2.0;
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x - follower_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left < 0.0 && target_x + target_width + follower_width > inner_width {
                    (target_x + follower_width, FollowerPlacement::Right)
                } else {
                    (left, FollowerPlacement::Left)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-50%)"),
                placement,
            })
        }
        FollowerPlacement::LeftStart => {
            let top = target_rect.y();
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x - follower_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left < 0.0 && target_x + target_width + follower_width > inner_width {
                    (target_x + follower_width, FollowerPlacement::RightStart)
                } else {
                    (left, FollowerPlacement::LeftStart)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::new(),
                placement,
            })
        }
        FollowerPlacement::LeftEnd => {
            let top = target_rect.y() + target_rect.height();
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x - follower_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left < 0.0 && target_x + target_width + follower_width > inner_width {
                    (target_x + follower_width, FollowerPlacement::RightEnd)
                } else {
                    (left, FollowerPlacement::LeftEnd)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-100%)"),
                placement,
            })
        }
        FollowerPlacement::Right => {
            let top = target_rect.y() + target_rect.height() / 2.0;
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x + target_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left + follower_width > inner_width && target_x - follower_width >= 0.0 {
                    (target_x - follower_width, FollowerPlacement::Left)
                } else {
                    (left, FollowerPlacement::Right)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-50%)"),
                placement,
            })
        }
        FollowerPlacement::RightStart => {
            let top = target_rect.y();
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x + target_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left + follower_width > inner_width && target_x - follower_width >= 0.0 {
                    (target_x - follower_width, FollowerPlacement::LeftStart)
                } else {
                    (left, FollowerPlacement::RightStart)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::new(),
                placement,
            })
        }
        FollowerPlacement::RightEnd => {
            let top = target_rect.y() + target_rect.height();
            let (left, placement) = {
                let follower_width = follower_rect.width();
                let target_x = target_rect.x();
                let target_width = target_rect.width();
                let left = target_x + target_width;

                let Some(inner_width) = window_inner_width() else {
                    return None;
                };

                if left + follower_width > inner_width && target_x - follower_width >= 0.0 {
                    (target_x - follower_width, FollowerPlacement::LeftEnd)
                } else {
                    (left, FollowerPlacement::RightEnd)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateY(-100%)"),
                placement,
            })
        }
        FollowerPlacement::Bottom => {
            let left = target_rect.x() + target_rect.width() / 2.0;
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y + target_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top + follower_height > inner_height && target_y - follower_height >= 0.0 {
                    (target_y - follower_height, FollowerPlacement::Top)
                } else {
                    (top, FollowerPlacement::Bottom)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-50%)"),
                placement,
            })
        }
        FollowerPlacement::BottomStart => {
            let left = target_rect.x();
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y + target_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top + follower_height > inner_height && target_y - follower_height >= 0.0 {
                    (target_y - follower_height, FollowerPlacement::TopStart)
                } else {
                    (top, FollowerPlacement::BottomStart)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::new(),
                placement,
            })
        }
        FollowerPlacement::BottomEnd => {
            let left = target_rect.x() + target_rect.width();
            let (top, placement) = {
                let follower_height = follower_rect.height();
                let target_y = target_rect.y();
                let target_height = target_rect.height();
                let top = target_y + target_height;

                let Some(inner_height) = window_inner_height() else {
                    return None;
                };

                if top + follower_height > inner_height && target_y - follower_height >= 0.0 {
                    (target_y - follower_height, FollowerPlacement::TopEnd)
                } else {
                    (top, FollowerPlacement::BottomEnd)
                }
            };
            Some(FollowerPlacementOffset {
                top,
                left,
                transform: String::from("translateX(-100%)"),
                placement,
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
