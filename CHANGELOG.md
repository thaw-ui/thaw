## [0.5.0-alpha](https://github.com/thaw-ui/thaw/compare/v0.4.5...v0.5.0-alpha) (2025-03-30)

### Features

* Adds `Tree` component, closes [#392](https://github.com/thaw-ui/thaw/pull/392).
* `Menu` is changed to a generic component, closes [#390](https://github.com/thaw-ui/thaw/pull/390).
 
### Breaking Changes

* Update leptos to v0.8.0-beta.
* Theme is changed to use getset.
* RatingColor adds Marigold prop.
* InputType adds Number prop.
* The icon prop of `Icon` is changed to icondata_core::Icon type.
* The value prop of `MenuItem` is changed to generic type.

## [0.4.5](https://github.com/thaw-ui/thaw/compare/v0.4.4...v0.4.5) (2025-03-30)

### Features

* `NavDrawer` adds open_categories prop, closes [#387](https://github.com/thaw-ui/thaw/pull/387).

## [0.4.4](https://github.com/thaw-ui/thaw/compare/v0.4.3...v0.4.4) (2025-03-02)

### Features

* Add click imperative handle to `Button`, closes [#377](https://github.com/thaw-ui/thaw/pull/377).
* Added autocomplete to `Input`, closes [#379](https://github.com/thaw-ui/thaw/pull/379).
* Add value prop to `Switch`, closes [#381](https://github.com/thaw-ui/thaw/pull/381).
* Adds `Persona` component, closes [#386](https://github.com/thaw-ui/thaw/pull/386).

### Bug Fixs

* Signal tracking problem in use_binder, closes [#383](https://github.com/thaw-ui/thaw/pull/383).

## [0.4.3](https://github.com/thaw-ui/thaw/compare/v0.4.2...v0.4.3) (2025-02-15)

### Features

* Update leptos to v0.7.7, closes [#375](https://github.com/thaw-ui/thaw/pull/375).

### Bug Fixs

* The default initial value of Input, closes [#373](https://github.com/thaw-ui/thaw/pull/373).

## [0.4.2](https://github.com/thaw-ui/thaw/compare/v0.4.1...v0.4.2) (2025-01-25)

### Features

* `Input` adds input_style prop, closes [#360](https://github.com/thaw-ui/thaw/pull/360).
* Adds `RangeSlider` component, closes [#366](https://github.com/thaw-ui/thaw/pull/366).
* `Slider` adds vertical prop.
* `Slider` adds show_stops prop.
* Adds `Rating` component, closes [#369](https://github.com/thaw-ui/thaw/pull/369).
* Adds `RatingDisplay` component, closes [#369](https://github.com/thaw-ui/thaw/pull/369).

### Bug Fixs

* Fix `Radio` and `Switch` component styles, closes [#362](https://github.com/thaw-ui/thaw/pull/362).

## [0.4.1](https://github.com/thaw-ui/thaw/compare/v0.4.0...v0.4.1) (2025-01-05)

### Features

* Add a select property for `InputRef`, closes [#358](https://github.com/thaw-ui/thaw/pull/358).

### Bug Fixs

* `SSRMountStyleProvider` hydration error, closes [#347](https://github.com/thaw-ui/thaw/pull/347).
* When the `Switch` changes the value, it is not displayed correctly, closes [#351](https://github.com/thaw-ui/thaw/pull/351).
* Fallback `Avatar` to initials on image load error, closes [#353](https://github.com/thaw-ui/thaw/pull/353).
* Panic when using `Input` inside `Suspense` under load, closes [#352](https://github.com/thaw-ui/thaw/pull/352).
* `Combobox` height, closes [#356](https://github.com/thaw-ui/thaw/pull/356).

## [0.4.0](https://github.com/thaw-ui/thaw/compare/v0.4.0-rc...v0.4.0) (2024-12-23)

### Features

- Update leptos to v0.7.2.
- `Avatar`: implement initials name, closes [#338](https://github.com/thaw-ui/thaw/pull/338).
- Custom themes, closes [#340](https://github.com/thaw-ui/thaw/pull/340).
- Input category component adds the size prop, closes [#341](https://github.com/thaw-ui/thaw/pull/341).

## Previous Changelogs

### 0.1.0-beta2 - 0.4.0-rc (2023-11-14 - 2024-12-09)

See [0.1 - 0.4 changelog](./changelogs/CHANGELOG-0.1-0.4.md)
