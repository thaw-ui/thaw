# Layout

```rust demo
view! {
    <Layout>
        <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">
            "Header"
        </LayoutHeader>
        <Layout style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
    </Layout>
}
```

### Sider

```rust demo
view! {
    <Layout has_sider=true>
        <LayoutSider style="background-color: #0078ff99; padding: 20px;">
            "Sider"
        </LayoutSider>
        <Layout>
            <LayoutHeader style="background-color: #0078ffaa; padding: 20px;">
                "Header"
            </LayoutHeader>
            <Layout style="background-color: #0078ff88; padding: 20px;">
                "Content"
            </Layout>
        </Layout>
    </Layout>
}
```

### Layout Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the layout element. |
| style | `MaybeSignal<String>` | `Default::default()` | Layout's style. |
| position | `LayoutPosition` | `LayoutPosition::Static` | static position will make it css position set to static. absolute position will make it css position set to absolute and left, right, top, bottom to 0. absolute position is very useful when you want to make content scroll in a fixed container or make the whole page's layout in a fixed position. You may need to change the style of the component to make it display as you expect. |
| children | `Children` |  | Layout's content. |

### LayoutHeader, LayoutSider Props

| Name     | Type                  | Default              | Description                                      |
| -------- | --------------------- | -------------------- | ------------------------------------------------ |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the layout header element. |
| style    | `MaybeSignal<String>` | `Default::default()` | LayoutHeader's style.                            |
| children | `Children`            |                      | LayoutHeader's content.                          |
