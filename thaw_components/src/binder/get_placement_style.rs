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
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
) -> Option<FollowerPlacementOffset> {
    use FollowerPlacement::*;
    let placement_list = match placement {
        TopStart => vec![TopStart, BottomStart, Right, Left, TopStart],
        Top => vec![Top, Bottom, Right, Left, Top],
        TopEnd => vec![TopEnd, BottomEnd, Right, Left, TopEnd],
        BottomStart => vec![BottomStart, TopStart, Right, Left, BottomStart],
        Bottom => vec![Bottom, Top, Right, Left, Bottom],
        BottomEnd => vec![BottomEnd, TopEnd, Right, Left, BottomEnd],
        RightStart => vec![RightStart, LeftStart, Top, Bottom, RightStart],
        Right => vec![Right, Left, Top, Bottom, Right],
        RightEnd => vec![RightEnd, LeftEnd, Top, Bottom, RightEnd],
        LeftStart => vec![LeftStart, RightStart, Top, Bottom, LeftStart],
        Left => vec![Left, Right, Top, Bottom, Left],
        LeftEnd => vec![LeftEnd, RightEnd, Top, Bottom, LeftEnd],
    };

    let placement_len = placement_list.len();
    let mut placement_list = placement_list.into_iter().enumerate();
    loop {
        let Some((index, placement)) = placement_list.next() else {
            return None;
        };
        let must = placement_len == index + 1;
        let rt = match placement {
            TopStart => placement_top_start(target_rect, content_rect, arrow_padding, must),
            Top => placement_top(target_rect, content_rect, arrow_padding, must),
            TopEnd => placement_top_end(target_rect, content_rect, arrow_padding, must),
            BottomStart => placement_bottom_start(target_rect, content_rect, arrow_padding, must),
            Bottom => placement_bottom(target_rect, content_rect, arrow_padding, must),
            BottomEnd => placement_bottom_end(target_rect, content_rect, arrow_padding, must),
            RightStart => placement_right_start(target_rect, content_rect, arrow_padding, must),
            Right => placement_right(target_rect, content_rect, arrow_padding, must),
            RightEnd => placement_right_end(target_rect, content_rect, arrow_padding, must),
            LeftStart => placement_left_start(target_rect, content_rect, arrow_padding, must),
            Left => placement_left(target_rect, content_rect, arrow_padding, must),
            LeftEnd => placement_left_end(target_rect, content_rect, arrow_padding, must),
        };
        if rt.is_some() {
            return rt;
        }
    }
    // Some(FollowerPlacementOffset {
    //     top: top - follower_rect.top(),
    //     left: left - follower_rect.left(),
    //     placement,
    //     transform,
    //     max_height,
    // })
}

fn placement_top_start(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_height = content_rect.height() + arrow_padding.unwrap_or_default();
    let target_top = target_rect.top();
    let top = target_top - content_height;
    // Top
    if !must && top < 0.0 {
        return None;
    }

    let target_left = target_rect.left();
    if !must {
        let Some(window_inner_width) = window_inner_width() else {
            return None;
        };
        let content_width = content_rect.width();
        // Width
        if target_left + content_width > window_inner_width {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top,
        left: target_left,
        transform: String::new(),
        placement: FollowerPlacement::TopStart,
        max_height: Some(target_top.max(0.0)),
    })
}

fn placement_top(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_height = content_rect.height() + arrow_padding.unwrap_or_default();
    let target_top = target_rect.top();
    let top = target_top - content_height;
    // Top
    if !must && top < 0.0 {
        return None;
    }

    let target_width_center = target_rect.left() + target_rect.width() / 2.0;
    if !must {
        let Some(window_inner_width) = window_inner_width() else {
            return None;
        };
        let content_width_half = content_rect.width() / 2.0;
        // Width
        if content_width_half > target_width_center
            || target_width_center + content_width_half > window_inner_width
        {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top,
        left: target_width_center,
        transform: String::from("translateX(-50%)"),
        placement: FollowerPlacement::Top,
        max_height: Some(target_top.max(0.0)),
    })
}

fn placement_top_end(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_height = content_rect.height() + arrow_padding.unwrap_or_default();
    let target_top = target_rect.top();
    let top = target_top - content_height;
    // Top
    if !must && top < 0.0 {
        return None;
    }

    let target_right = target_rect.right();
    if !must {
        let content_width = content_rect.width();
        // Width
        if target_right < content_width {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top,
        left: target_right,
        transform: String::from("translateX(-100%)"),
        placement: FollowerPlacement::TopEnd,
        max_height: Some(target_top.max(0.0)),
    })
}

fn placement_bottom_start(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_height) = window_inner_height() else {
        return None;
    };
    let content_height = content_rect.height();
    let target_bottom = target_rect.bottom() + arrow_padding.unwrap_or_default();
    let top = target_bottom;
    // Bottom
    if !must && target_bottom + content_height > window_inner_height {
        return None;
    }

    let target_left = target_rect.left();
    if !must {
        let Some(window_inner_width) = window_inner_width() else {
            return None;
        };
        let content_width = content_rect.width();
        // Width
        if target_left + content_width > window_inner_width {
            return None;
        }
    }

    let max_heigth = if target_bottom > 0.0 {
        window_inner_height - target_bottom
    } else {
        0.0
    };
    Some(FollowerPlacementOffset {
        top,
        left: target_left,
        transform: String::new(),
        placement: FollowerPlacement::BottomStart,
        max_height: Some(max_heigth),
    })
}

fn placement_bottom(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_height) = window_inner_height() else {
        return None;
    };
    let content_height = content_rect.height();
    let target_bottom = target_rect.bottom() + arrow_padding.unwrap_or_default();
    let top = target_bottom;
    // Bottom
    if !must && target_bottom + content_height > window_inner_height {
        return None;
    }

    let target_width_center = target_rect.left() + target_rect.width() / 2.0;
    if !must {
        let Some(window_inner_width) = window_inner_width() else {
            return None;
        };
        let content_width_half = content_rect.width() / 2.0;
        // Width
        if content_width_half > target_width_center
            || target_width_center + content_width_half > window_inner_width
        {
            return None;
        }
    }

    let max_heigth = if target_bottom > 0.0 {
        window_inner_height - target_bottom
    } else {
        0.0
    };

    Some(FollowerPlacementOffset {
        top,
        left: target_width_center,
        transform: String::from("translateX(-50%)"),
        placement: FollowerPlacement::Bottom,
        max_height: Some(max_heigth),
    })
}

fn placement_bottom_end(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_height) = window_inner_height() else {
        return None;
    };
    let content_height = content_rect.height();
    let target_bottom = target_rect.bottom() + arrow_padding.unwrap_or_default();
    let top = target_bottom;
    // Bottom
    if !must && target_bottom + content_height > window_inner_height {
        return None;
    }

    let target_right = target_rect.right();
    if !must {
        let content_width = content_rect.width();
        // Width
        if target_right < content_width {
            return None;
        }
    }

    let max_heigth = if target_bottom > 0.0 {
        window_inner_height - target_bottom
    } else {
        0.0
    };
    Some(FollowerPlacementOffset {
        top,
        left: target_right,
        transform: String::from("translateX(-100%)"),
        placement: FollowerPlacement::BottomEnd,
        max_height: Some(max_heigth),
    })
}

fn placement_right_start(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_width) = window_inner_width() else {
        return None;
    };
    let content_width = content_rect.width();
    let target_right = target_rect.right();
    let left = target_right + arrow_padding.unwrap_or_default();
    // Right
    if !must && left + content_width > window_inner_width {
        return None;
    }

    let top = target_rect.top();
    if !must {
        let Some(window_inner_height) = window_inner_height() else {
            return None;
        };
        let content_height = content_rect.height();
        // Height
        if content_height + top > window_inner_height {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top,
        left,
        transform: String::new(),
        placement: FollowerPlacement::RightStart,
        max_height: None,
    })
}

fn placement_right(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_width) = window_inner_width() else {
        return None;
    };
    let content_width = content_rect.width();
    let target_right = target_rect.right();
    let left = target_right + arrow_padding.unwrap_or_default();
    // Right
    if !must && left + content_width > window_inner_width {
        return None;
    }

    let target_height_center = target_rect.top() + target_rect.height() / 2.0;
    if !must {
        let Some(window_inner_height) = window_inner_height() else {
            return None;
        };
        let content_height_half = content_rect.height() / 2.0;

        // Height
        if content_height_half > target_height_center
            || target_height_center + content_height_half > window_inner_height
        {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top: target_height_center,
        left,
        transform: String::from("translateY(-50%)"),
        placement: FollowerPlacement::Right,
        max_height: None,
    })
}

fn placement_right_end(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let Some(window_inner_width) = window_inner_width() else {
        return None;
    };
    let content_width = content_rect.width();
    let target_right = target_rect.right();
    let left = target_right + arrow_padding.unwrap_or_default();
    // Right
    if !must && left + content_width > window_inner_width {
        return None;
    }

    let target_bottom = target_rect.bottom();
    if !must {
        // Height
        if target_bottom < content_rect.height() {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top: target_bottom,
        left,
        transform: String::from("translateY(-100%)"),
        placement: FollowerPlacement::RightEnd,
        max_height: None,
    })
}

fn placement_left_start(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_width = content_rect.width() + arrow_padding.unwrap_or_default();
    let target_left = target_rect.left();
    let left = target_left - content_width;
    // Left
    if !must && left < 0.0 {
        return None;
    }

    let top = target_rect.top();
    if !must {
        let Some(window_inner_height) = window_inner_height() else {
            return None;
        };
        let content_height = content_rect.height();
        // Height
        if content_height + top > window_inner_height {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top,
        left,
        transform: String::new(),
        placement: FollowerPlacement::LeftStart,
        max_height: None,
    })
}

fn placement_left(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_width = content_rect.width() + arrow_padding.unwrap_or_default();
    let target_left = target_rect.left();
    let left = target_left - content_width;
    // Left
    if !must && left < 0.0 {
        return None;
    }

    let target_height_center = target_rect.top() + target_rect.height() / 2.0;
    if !must {
        let Some(window_inner_height) = window_inner_height() else {
            return None;
        };
        let content_height_half = content_rect.height() / 2.0;

        // Height
        if content_height_half > target_height_center
            || target_height_center + content_height_half > window_inner_height
        {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top: target_height_center,
        left,
        transform: String::from("translateY(-50%)"),
        placement: FollowerPlacement::Left,
        max_height: None,
    })
}

fn placement_left_end(
    target_rect: &DomRect,
    content_rect: &DomRect,
    arrow_padding: Option<f64>,
    must: bool,
) -> Option<FollowerPlacementOffset> {
    let content_width = content_rect.width() + arrow_padding.unwrap_or_default();
    let target_left = target_rect.left();
    let left = target_left - content_width;
    // Left
    if !must && left < 0.0 {
        return None;
    }

    let target_bottom = target_rect.bottom();
    if !must {
        // Height
        if target_bottom < content_rect.height() {
            return None;
        }
    }

    Some(FollowerPlacementOffset {
        top: target_bottom,
        left,
        transform: String::from("translateY(-100%)"),
        placement: FollowerPlacement::LeftEnd,
        max_height: None,
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
