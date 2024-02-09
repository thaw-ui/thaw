# Color Picker

```rust demo
use palette::Srgb;

let value = create_rw_signal(Color::from(Srgb::new(0.0, 0.0, 0.0)));

view! {
    <ColorPicker value/>
}
```

### Color Format

Encoding formats, support RGB, HSV, HSL.

```rust demo
use palette::{Hsl, Hsv, Srgb};

let rgb = create_rw_signal(Color::from(Srgb::new(0.0, 0.0, 0.0)));
let hsv = create_rw_signal(Color::from(Hsv::new(0.0, 0.0, 0.0)));
let hsl = create_rw_signal(Color::from(Hsl::new(0.0, 0.0, 0.0)));

view! {
    <Space vertical=true>
        <ColorPicker value=rgb/>
        <ColorPicker value=hsv/>
        <ColorPicker value=hsl/>
    </Space>
}
```

### ColorPicker Props

| Name  | Type                                | Default              | Desciption                                      |
| ----- | ----------------------------------- | -------------------- | ----------------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the color picker element. |
| value | `Model<Color>`                      | `Default::default()` | Value of the picker.                            |
