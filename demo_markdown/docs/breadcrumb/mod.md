# Breadcrumb

```rust demo
view! {
    <Breadcrumb>
        <BreadcrumbItem>"Leptos"</BreadcrumbItem>
        <BreadcrumbItem>"UI"</BreadcrumbItem>
        <BreadcrumbItem>"Thaw"</BreadcrumbItem>
    </Breadcrumb>
}
```

### Separator

```rust demo
view! {
    <Breadcrumb separator=">">
        <BreadcrumbItem>"Leptos"</BreadcrumbItem>
        <BreadcrumbItem>"UI"</BreadcrumbItem>
        <BreadcrumbItem>"Thaw"</BreadcrumbItem>
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
