# Pagination

```rust demo
let (page, set_page) = create_signal(1);

view! {
    <Space vertical=true>
        <div>"Page: " {move || page.get()}</div>
        <Pagination page on_change=move |p| set_page.set(p) count=10 />
    </Space>
}
```

### Color

```rust demo
view! {
    <Space vertical=true>
        <Pagination count=10 color=ButtonColor::Primary />
        <Pagination count=10 color=ButtonColor::Success />
        <Pagination count=10 color=ButtonColor::Warning />
        <Pagination count=10 color=ButtonColor::Error />
    </Space>
}
```

### Size

```rust demo
view! {
    <Space vertical=true>
        <Pagination count=100 size=ButtonSize::Tiny />
        <Pagination count=100 size=ButtonSize::Small />
        <Pagination count=100 size=ButtonSize::Medium />
        <Pagination count=100 size=ButtonSize::Large />
    </Space>
}
```

### Pagination ranges

```rust demo
view! {
    <Space vertical=true>
        <Pagination count=100 sibling_count=0 />
        <Pagination count=100 sibling_count=1 />
        <Pagination count=100 sibling_count=2 />
        <Pagination count=100 sibling_count=3 />
    </Space>
}
```

### Pagination Props

| Name          | Type                                | Default                | Description                                                |
| ------------- | ----------------------------------- | ---------------------- | ---------------------------------------------------------- |
| class         | `OptionalProp<MaybeSignal<String>>` | `Default::default()`   | Additional classes.                                        |
| page          | `MaybeSignal<i64>`                  | `1`                    | The current page starts from 1.                            |
| count         | `MaybeSignal<i64>`                  |                        | The total numbers of pages.                                |
| sibling_count | `MaybeSignal<i64>`                  | `1`                    | Number of visible pages after and before the current page. |
| color         | `MaybeSignal<ButtonColor>`          | `ButtonColor::Primary` | Button's color.                                            |
| size          | `MaybeSignal<ButtonSize>`           | `ButtonSize::Medium`   | Button size.                                               |
| on_change     | `Option<Callback<i64>>`             | `None`                 | Callback fired when the page is changed.                   |
