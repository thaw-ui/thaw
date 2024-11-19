# Pagination

```rust demo
let page = RwSignal::new(1);

view! {
    <Space vertical=true>
        <div>"Page: " {move || page.get()}</div>
        <Pagination page page_count=10 />
    </Space>
}
```

### Pagination ranges

```rust demo
view! {
    <Space vertical=true>
        <Pagination page_count=100 sibling_count=0 />
        <Pagination page_count=100 sibling_count=1 />
        <Pagination page_count=100 sibling_count=2 />
        <Pagination page_count=100 sibling_count=3 />
    </Space>
}
```

### Pagination Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| page | `Model<usize>` | `1` | The current page starts from 1. |
| page_count | `Signal<usize>` |  | The total numbers of pages. |
| sibling_count | `Signal<usize>` | `1` | Number of visible pages after and before the current page. |
