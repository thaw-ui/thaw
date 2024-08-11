# Spinner

```rust demo
view! {
    <Spinner/>
}
```

### Size

```rust demo
view! {
    <Space vertical=true>
        <Spinner size=SpinnerSize::ExtraTiny label="Extra Tiny Spinner"/>
        <Spinner size=SpinnerSize::Tiny label="Tiny Spinner"/>
        <Spinner size=SpinnerSize::ExtraSmall label="Extra Small Spinner"/>
        <Spinner size=SpinnerSize::Small label="Small Spinner"/>
        <Spinner size=SpinnerSize::Medium label="Medium Spinner"/>
        <Spinner size=SpinnerSize::Large label="Large Spinner"/>
        <Spinner size=SpinnerSize::ExtraLarge label="Extra Large Spinner"/>
        <Spinner size=SpinnerSize::Huge label="Huge Spinner"/>
    </Space>
}
```

### Spinner Props

| Name  | Type                                | Default               | Description                                 |
| ----- | ----------------------------------- | --------------------- | ------------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()`  | Additional classes for the spinner element. |
| size  | `MaybeSignal<SpinnerSize>`          | `SpinnerSize::Medium` | Spinner size.                               |
