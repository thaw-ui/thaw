# Pagination

```rust demo
let page = create_rw_signal(1);

view! {
    <Space vertical=true>
        <div>"Page: " {move || page.get()}</div>
        <Pagination page count=10 />
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

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Additional classes. |
| page | `Model<usize>` | `1` | The current page starts from 1. |
| count | `MaybeSignal<usize>` |  | The total numbers of pages. |
| sibling_count | `MaybeSignal<usize>` | `1` | Number of visible pages after and before the current page. |
| size | `MaybeSignal<ButtonSize>` | `ButtonSize::Medium` | Button size. |
| on_change | `Option<Callback<usize>>` | `None` | Callback fired when the page is changed. |
