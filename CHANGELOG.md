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
