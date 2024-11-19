# Breadcrumb

```rust demo
view! {
    <Breadcrumb>
        <BreadcrumbItem>
            <BreadcrumbButton>"Leptos"</BreadcrumbButton>
        </BreadcrumbItem>
        <BreadcrumbDivider />
        <BreadcrumbItem>
            <BreadcrumbButton>"UI"</BreadcrumbButton>
        </BreadcrumbItem>
        <BreadcrumbDivider />
        <BreadcrumbItem>
            <BreadcrumbButton current=true>"Thaw"</BreadcrumbButton>
        </BreadcrumbItem>
    </Breadcrumb>
}
```

### Breadcrumb Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### BreadcrumbItem Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### BreadcrumbButton Props

| Name     | Type                | Default              | Description                               |
| -------- | ------------------- | -------------------- | ----------------------------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                                           |
| current  | `Signal<bool>`      | `false`              | Defines current sate of BreadcrumbButton. |
| children | `Children`          |                      |                                           |

### BreadcrumbDivider Props

| Name  | Type                | Default              | Description |
| ----- | ------------------- | -------------------- | ----------- |
| class | `MaybeProp<String>` | `Default::default()` |             |
