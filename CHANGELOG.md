## [0.4.0-beta4](https://github.com/thaw-ui/thaw/compare/v0.4.0-beta3...v0.4.0-beta4) (2024-10-28)

### Features

* `OverlayDrawer` adds modal_type prop.
* `Field` adds required prop.
* Adss `Label` component.
* ToastOptions adds intent, dismiss and on_status_change.
* `Table` resizable.
* When the `Field` status is Error, the border of the input class component is displayed in red.

### Bug Fixs

* `OverlayDrawer`: only set aria-modal to true when modal.
* `TagPicker` clears input when value is selected.

### Breaking Changes

* Change the any_view prop of dispatch_toast to children, closes [#267](https://github.com/thaw-ui/thaw/pull/267).
* apply the OverlayDrawer class prop to thaw-overlay-drawer, closes [#278](https://github.com/thaw-ui/thaw/pull/278).
* Update leptos to v0.7.0-rc0.

## [0.4.0-beta3](https://github.com/thaw-ui/thaw/compare/v0.4.0-beta2...v0.4.0-beta3) (2024-09-13)

### Features

* Adss `Flex` component.
* Adss `TagPicker` component.
* `Spinner` adds children prop.

### Bug Fixs

* `AnchorLink` outline.
* `SliderLabel` position.
* `NavDrawer` selected category value error.
* Adjust the z-index of the Binder popup.
* Binder component display position.

### Breaking Changes

* Change Tag's closable to dismissible.
* Update leptos to v0.7.0-beta5.

## [0.3.4](https://github.com/thaw-ui/thaw/compare/v0.3.3...v0.3.4) (2024-09-11)

### Features

* Adds `Flex` component.
* Update leptos to v0.6.14.

### Bug Fixs

* Allow creating custom theme [#242](https://github.com/thaw-ui/thaw/pull/242).
* `CheckboxGroup` does not display default value.
* `Binder` component display position.

## [0.4.0-beta2](https://github.com/thaw-ui/thaw/compare/v0.4.0-beta...v0.4.0-beta2) (2024-09-02)

### Features

* Font add ui-* and system-ui.
* Add id, name and rules to the input class component.
* `Calendar` adds children prop.
* `Button` adds button_type prop.
* Adds `Field` component.
* Adss `Link` component.
* Adss `Select` component.

### Bug Fixs

* Scrollbar nested update value.
* The position is messed up when switching DatePicker panels.
* Clicking the ExpandIcon of the Combobox does not close the popup window.

### Breaking Changes

* Change the type of dir in ConfigProvider.
* Update leptos to v0.7.0-beta4.

## [0.3.3](https://github.com/thaw-ui/thaw/compare/v0.3.2...v0.3.3) (2024-07-15)

### Features

* `MenuItem` adds icon prop, closes [#200](https://github.com/thaw-ui/thaw/pull/200).
* `MenuItem` adds children prop, closes [#207](https://github.com/thaw-ui/thaw/pull/207).
* `Modal` adds closable prop, closes [#206](https://github.com/thaw-ui/thaw/pull/206).
* Adds `Dropdown` component, closes [#210](https://github.com/thaw-ui/thaw/pull/210).
* Added parser and formatter props to input and `InputNumber` components, closes [#209](https://github.com/thaw-ui/thaw/pull/209).
* Adds `Pagination` component, closes [#212](https://github.com/thaw-ui/thaw/pull/212).

### Bug Fixs

* use_lock_html_scroll, closes [#215](https://github.com/thaw-ui/thaw/pull/215).
* Events were not removed when the scrollbar cleared, closes [#216](https://github.com/thaw-ui/thaw/pull/216).

## [0.3.2](https://github.com/thaw-ui/thaw/compare/v0.3.1...v0.3.2) (2024-05-21)

### Features

* `Switch` adds on_change prop, closes [#196](https://github.com/thaw-ui/thaw/pull/196).

### Bug Fixs

* `Image` object-fit, closes [#195](https://github.com/thaw-ui/thaw/pull/195).
* `Icon` missaligment in message fix, closes [#194](https://github.com/thaw-ui/thaw/pull/194).
* fix `Scrollbar` panic, closes [#193](https://github.com/thaw-ui/thaw/pull/193).
* interchange the icons of MessageVariant::Success and MessageVariant::Error, closes [#188](https://github.com/thaw-ui/thaw/pull/188).

## [0.3.1](https://github.com/thaw-ui/thaw/compare/v0.3.0...v0.3.1) (2024-04-27)

### Features

* Adds `BackTop` component.
* Adds `Anchor` component.
* Adds `MultiSelect` component.
* thaw_utils: Adds get_scroll_parent function.
* thaw_utils: Adds add_event_listener_with_bool function.
* thaw_utils: Adds throttle function.

### Bug Fixs

* `Button` font size styling.
* `TimePicker` scroll problem.
* `Icon` inline-block.
* `Select` height problem.

## [0.3.0](https://github.com/thaw-ui/thaw/compare/v0.3.0-beta...v0.3.0) (2024-04-14)

### Features

* `Popover` adds tooltip prop.

### Bug Fixs

* ssr mode `Input` default value.

## [0.3.0-beta](https://github.com/thaw-ui/thaw/compare/v0.3.0-alpha...v0.3.0-beta) (2024-04-10)

### Breaking Changes

* `Drawer` adds close_on_esc and mask_closeable prop.
* `Drawer` scrollbar.
* `Modal` scrollbar.
* `Teleport` related component will lazy mount. (Defer rendering of the DOM until it is first displayed)

### Features

* thaw_components: Adds `FocusTrap` component.
* thaw_components: `Teleport` adds immediate prop.

### Bug Fixs

* `Modal` esc close problem.

## [0.3.0-alpha](https://github.com/thaw-ui/thaw/compare/v0.2.6...v0.3.0-alpha) (2024-04-05)

### Breaking Changes

* `Checkbox` label does not display default value.
* `Modal` adds close_on_esc prop.
* Add min/max props into `InputNumber`.
* `Layout` scrollbar. (When class or style is used, the original style may be confused.  You can use content_class or content_style instead)

### Features

* Adds `Scrollbar` component.
* `TimePicker` scrollbar.
* `Layout` adds content_class and content_style prop.
* Update leptos to v0.6.10.

## [0.2.6](https://github.com/thaw-ui/thaw/compare/v0.2.5...v0.2.6) (2024-03-31)

### Features

* Optimize `Switch` styles.
* `Message` adds animation.
* thaw_components: `CSSTransition` adds appear, on_before_enter, on_before_leave and on_leave prop.

### Bug Fixs

* `Button` wave flicker problem.

## [0.2.5](https://github.com/thaw-ui/thaw/compare/v0.2.3...v0.2.5) (2024-03-20)

### Features

* Added thaw_components library.
* Added thaw_utils library.
* Update leptos to v0.6.9.
* `Button` adds block prop.
* `ButtonGroup` adds class prop.
* `Tag` adds closable and on_close prop.
* `AutoComplete` adds blur_after_select prop.
* `Space` adds align and justify prop.

### Bug Fixs

* `Toast` cfg in ssr mode.
* `CSSTransition` leakage in the next_frame function.
* `Icon` rendering problem in ssr mode.

## 0.2.4 unpublish

## [0.2.3](https://github.com/thaw-ui/thaw/compare/v0.2.2...v0.2.3) (2024-03-07)

### Features

* Update leptos to v0.6.7.
* Open animation.
* `Button` icon property supports the signal.

### Bug Fixs

* `Icon` returns the function's warning.

## [0.2.2](https://github.com/thaw-ui/thaw/compare/v0.2.1...v0.2.2) (2024-02-29)

### Features

* Adds `ProgressCircle` component.

### Bug Fixs

* `Icon` fill default value.
* `Drawer` cannot be closed when the placement is changed.

## [0.2.1](https://github.com/thaw-ui/thaw/compare/v0.2.0...v0.2.1) (2024-02-25)

### Features

* Adds `RadioGroup` component.
* `AutoComplete` adds allow_free_input prop.

### Bug Fixs

* `Drawer` cannot be closed when the placement is changed.
* `Icon` support event.
* `Modal` nightly build problem.

## [0.2.0](https://github.com/thaw-ui/thaw/compare/v0.2.0-beta...v0.2.0) (2024-02-21)

### Breaking Changes

* `Modal` will be closed when clicking on the mask.

### Features

* `Modal` adds mask_closeable, width and z_index prop.

## [0.2.0-beta](https://github.com/thaw-ui/thaw/compare/v0.2.0-alpha...v0.2.0-beta) (2024-02-07)

### Breaking Changes

* `ColorPicker` changes the value type to Color.
* Remove re-export of chrono.

### Features

* `Drawer` adds z_index and mount prop.

## [0.2.0-alpha](https://github.com/thaw-ui/thaw/compare/v0.1.10...v0.2.0-alpha) (2024-02-01)

### Breaking Changes

* Remove the default feature of csr.
* `ButtonVariant::Solid` is changed to `ButtonVariant::Outlined`.
* Change icondata to icondata_core.
* Rewrite `Code` component.
* Update leptos to v0.6.5.

### Features

* Change `RwSignal` to `Model`.
* Change `#[prop(optional)] T` to `#[prop(optional)] OptionalProp<T>`.
* `Checkbox` children can be empty.

## [0.1.10](https://github.com/thaw-ui/thaw/compare/v0.1.9...v0.1.10) (2024-01-31)

### Features

* `Tab` adds `TabLabel` slot.

## [0.1.9](https://github.com/thaw-ui/thaw/compare/v0.1.8...v0.1.9) (2024-01-24)

### Features

* `Message` adds close button and position.
* Update leptos to v0.5.7.

### Bug Fixs

* `Input` click edge does not focus.
* Label cannot be removed correctly when `Tab` is removed.

## [0.1.8](https://github.com/thaw-ui/thaw/compare/v0.1.7...v0.1.8) (2024-01-17)

### Features

* `CollapseItem` add animation.
* Adds `TextArea` component.
* `NavBar` adds `NavBarLeft` and `NavBarRight` slot.
* Update leptos to v0.5.6

### Bug Fixs

* `CheckboxItem` adds tracking value changes.

## [0.1.7](https://github.com/thaw-ui/thaw/compare/v0.1.6...v0.1.7) (2024-01-09)

### Features

* `InputNumber` adds ComponentRef.
* Adds `Drawer` component.
* Adds `Collapse` component.

## [0.1.6](https://github.com/thaw-ui/thaw/compare/v0.1.5...v0.1.6) (2024-01-02)

### Features

* `AutoComplete`, `DatePicker`, `Input`, `InputNumber` and `TimePicker` adds attrs prop.

## [0.1.5](https://github.com/thaw-ui/thaw/compare/v0.1.4...v0.1.5) (2023-12-26)

### Features

* Adds class param.

## [0.1.4](https://github.com/thaw-ui/thaw/compare/v0.1.3...v0.1.4) (2023-12-19)

### Features

* `Input` and `AutoComplete` adds ComponentRef.
* `AutoComplete` adds `AutoCompletePrefix` and `AutoCompleteSuffix` slot.
* Adds `Spinner` component.
* Adds `DatePicker` component.

## [0.1.3](https://github.com/thaw-ui/thaw/compare/v0.1.2...v0.1.3) (2023-12-12)

### Features

* **button:** Add button group component
* **button:** Add size property
* **button:** Add circle property
* **calendar:** Add calendar component
* **time picker:** Add time picker component
* **class:** Add class param to first components
* **slider:** Add step property
* **slider:** Add slider label component
 
### Bug Fixs

* **button:** The round property shows the problem

## [0.1.2](https://github.com/thaw-ui/thaw/compare/v0.1.1...v0.1.2) (2023-12-04)

### Features

* **leptos:** Update leptos to v0.5.4
* **auto complete:** Auto complete component add keyboard event
* **menu:** Menu item add line height
* **button:** Button add disabled style
* **disabled and invalid:** Add disabled and invalid statuses to input

### Bug Fixs

* **callback:** Nightly Callback conversion problem

## [0.1.1](https://github.com/thaw-ui/thaw/compare/v0.1.0...v0.1.1) (2023-11-27)

### Features

* **ssr:** Add ssr and hydrate
* **style:** GlobalStyle component margin style
* **component:** add `Text` component

## [0.1.0](https://github.com/thaw-ui/thaw/compare/v0.1.0-beta4...v0.1.0) (2023-11-19)

### Bug Fixs

* **provide_context:** Use the Provider component. Specific reasons see https://github.com/leptos-rs/leptos/issues/2038.

## [0.1.0-beta4](https://github.com/thaw-ui/thaw/compare/v0.1.0-beta3...v0.1.0-beta4) (2023-11-16)

* **input:** Input component add prefix slot.
* **progress:** The variant property of the progress component is changed to color.

## [0.1.0-beta3](https://github.com/thaw-ui/thaw/compare/v0.1.0-beta2...v0.1.0-beta3) (2023-11-15)

**rewrite:** Rewrite the progress component.

## [0.1.0-beta2](https://github.com/thaw-ui/thaw/compare/v0.1.0-beta...v0.1.0-beta2) (2023-11-14)

**private**: AlertVariant, ButtonColor, TagVariant theme related methods cancel pub.

**avatar**: The name of circle is changed to round and the type of size is changed to u16.

**badge**: The name of color is changed to variant, and max_value is changed to max.

**button**: The type of children is changed to Children.

**grid**: Change the type x_gap, y_gap, offset to u16.
