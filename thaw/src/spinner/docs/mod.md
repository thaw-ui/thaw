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

### Custom label

```rust demo
view! {
    <Spinner label="Label"/>
    <Spinner>
        <b style="color: blue">"Label"</b>
    </Spinner>
}
```

### Spinner Props

| Name     | Type                       | Default               | Description                        |
| -------- | -------------------------- | --------------------- | ---------------------------------- |
| class    | `MaybeProp<String>`        | `Default::default()`  |                                    |
| label    | `MaybeProp<String>`        | `Default::default()`  | An optional label for the Spinner. |
| size     | `MaybeSignal<SpinnerSize>` | `SpinnerSize::Medium` | The size of the spinner.           |
| children | `Option<Children>`         | `None`                |                                    |
