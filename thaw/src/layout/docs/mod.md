# Layout

```rust demo
view! {
    <Layout>
        <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
            "Header"
        </LayoutHeader>
        <Layout attr:style="background-color: #0078ff88; padding: 20px;">"Content"</Layout>
    </Layout>
}
```

### Sider

```rust demo
view! {
    <Layout has_sider=true>
        <LayoutSider attr:style="background-color: #0078ff99; padding: 20px;">
            "Sider"
        </LayoutSider>
        <Layout>
            <LayoutHeader attr:style="background-color: #0078ffaa; padding: 20px;">
                "Header"
            </LayoutHeader>
            <Layout attr:style="background-color: #0078ff88; padding: 20px;">
                "Content"
            </Layout>
        </Layout>
    </Layout>
}
```

### Layout Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| content_class | `MaybeProp<String>` | `Default::default()` | Addtional classes for the layout element. |
| content_style | `MaybeProp<String>` | `Default::default()` | Style of scrollable content node. |
| position | `LayoutPosition` | `LayoutPosition::Static` | static position will make it css position set to static. absolute position will make it css position set to absolute and left, right, top, bottom to 0. absolute position is very useful when you want to make content scroll in a fixed container or make the whole page's layout in a fixed position. You may need to change the style of the component to make it display as you expect. |
| has_sider | `MaybeSignal<bool>` | `false` | Whether the component has sider inside. If so it must be true. |
| children | `Children` |  |  |

### LayoutHeader Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### LayoutSider Props

| Name          | Type                | Default              | Description                               |
| ------------- | ------------------- | -------------------- | ----------------------------------------- |
| class         | `MaybeProp<String>` | `Default::default()` |                                           |
| content_class | `MaybeProp<String>` | `Default::default()` | Addtional classes for the layout element. |
| content_style | `MaybeProp<String>` | `Default::default()` | Style of scrollable content node.         |
| children      | `Children`          |                      |                                           |
