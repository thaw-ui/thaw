# Color Picker

```rust demo
let value = create_rw_signal(RGBA::default());

view! {
    <ColorPicker value/>
}
```

### DatePicker Props

| Name  | Type                  | Default              | Desciption                                      |
| ----- | --------------------- | -------------------- | ----------------------------------------------- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the color picker element. |
| value | `RwSignal<RGBA>`      | `Default::default()` | Value of the picker.                            |
