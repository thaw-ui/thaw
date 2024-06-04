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

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the breadcrumb element. |
| separator | `MaybeSignal<String>` | `/` | Breadcrumb separator. |
| children | `Children` |  | Breadcrumb's content. |

### BreadcrumbItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the breadcrumb link element. |
| children | `Children` |  | BreadcrumbItem's content. |
