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
