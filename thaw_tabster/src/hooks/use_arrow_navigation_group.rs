use super::{use_tabster::use_tabster, use_tabster_attributes::use_tabster_attributes};
use tabster::{
    get_mover, mover_directions,
    types::{self, MoverProps, TabsterAttributeProps},
};

pub enum Axis {
    Vertical,
    Horizontal,
    Grid,
    GridLinear,
    Both,
}

#[derive(Default)]
pub struct UseArrowNavigationGroupOptions {
    /// Focus will navigate vertically, horizontally or in both directions (grid), defaults to horizontally
    /// @defaultValue vertical
    pub axis: Option<Axis>,
    /// Focus will cycle to the first/last elements of the group without stopping
    pub circular: Option<bool>,
    /// Last focused element in the group will be remembered and focused (if still
    /// available) when tabbing from outside of the group
    /// @default true
    pub memorize_current: Option<bool>,
    /// Allow tabbing within the arrow navigation group items.
    pub tabbable: Option<bool>,
    /// Tabster should ignore default handling of keydown events
    // ignoreDefaultKeydown?: Types.FocusableProps['ignoreKeydown'];

    /// The default focusable item in the group will be an element with Focusable.isDefault property.
    /// Note that there is no way in \@fluentui/react-tabster to set default focusable element,
    /// and this option is currently for internal testing purposes only.
    pub unstable_has_default: Option<bool>,
}

/// A hook that returns the necessary tabster attributes to support arrow key navigation
/// @param options - Options to configure keyboard navigation
pub fn use_arrow_navigation_group(
    options: UseArrowNavigationGroupOptions,
) -> types::TabsterDOMAttribute {
    // const {
    //   circular,
    //   axis,
    //   memorizeCurrent = true,
    //   tabbable,
    //   ignoreDefaultKeydown,
    //   unstable_hasDefault,
    // } = options;
    let UseArrowNavigationGroupOptions {
        circular,
        tabbable,
        unstable_has_default,
        memorize_current,
        axis,
    } = options;
    let tabster = use_tabster();
    get_mover(&tabster);

    use_tabster_attributes(&TabsterAttributeProps {
        root: None,
        groupper: None,
        sys: None,
        mover: Some(MoverProps {
            direction: Some(axis_to_mover_direction(axis.unwrap_or(Axis::Vertical))),
            memorize_current,
            tabbable,
            cyclic: Some(circular.unwrap_or_default()),
            track_state: None,
            has_default: unstable_has_default,
            visibility_tolerance: None,
        }),
    })
    //   mover: {
    //     direction: axisToMoverDirection(axis ?? 'vertical'),
    //   },
    //   ...(ignoreDefaultKeydown && {
    //     focusable: {
    //       ignoreKeydown: ignoreDefaultKeydown,
    //     },
    //   }),
}

pub fn axis_to_mover_direction(axis: Axis) -> types::MoverDirection {
    match axis {
        Axis::Vertical => mover_directions::VERTICAL,
        Axis::Horizontal => mover_directions::HORIZONTAL,
        Axis::Grid => mover_directions::GRID,
        Axis::GridLinear => mover_directions::GRID_LINEAR,
        Axis::Both => mover_directions::BOTH,
    }
}
