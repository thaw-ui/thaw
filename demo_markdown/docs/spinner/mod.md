# Spinner

```rust demo
view! {
    <Spinner/>
}
```

### Size

```rust demo
view! {
    <Space>
        <Spinner size=SpinnerSize::Tiny/>
        <Spinner size=SpinnerSize::Small/>
        <Spinner size=SpinnerSize::Medium/>
        <Spinner size=SpinnerSize::Large/>
    </Space>
}
```

### Spinner Props

| Name  | Type                       | Default               | Description                                 |
| ----- | -------------------------- | --------------------- | ------------------------------------------- |
| class | `MaybeSignal<String>`      | `Default::default()`  | Additional classes for the spinner element. |
| size  | `MaybeSignal<SpinnerSize>` | `SpinnerSize::Medium` | Spinner size.                               |
