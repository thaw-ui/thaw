# Accordion

```rust demo
view! {
    <Accordion>
        <AccordionItem value="leptos">
            <AccordionHeader slot>
                "Leptos"
            </AccordionHeader>
            "Build fast web applications with Rust."
        </AccordionItem>
        <AccordionItem  value="thaw">
            <AccordionHeader slot>
                "Thaw"
            </AccordionHeader>
            "An easy to use leptos component library"
        </AccordionItem>
    </Accordion>
}
```

### Collapsible

An accordion can have multiple panels collapsed at the same time.

```rust demo
view! {
    <Accordion collapsible=true>
        <AccordionItem value="leptos">
            <AccordionHeader slot>
                "Leptos"
            </AccordionHeader>
            "Build fast web applications with Rust."
        </AccordionItem>
        <AccordionItem  value="thaw">
            <AccordionHeader slot>
                "Thaw"
            </AccordionHeader>
            "An easy to use leptos component library"
        </AccordionItem>
    </Accordion>
}
```

### Multiple

An accordion supports multiple panels expanded simultaneously. Since it's not simple to determine which panel will be opened by default, multiple will also be collapsed by default on the first render.

```rust demo
view! {
    <Accordion multiple=true>
        <AccordionItem value="leptos">
            <AccordionHeader slot>
                "Leptos"
            </AccordionHeader>
            "Build fast web applications with Rust."
        </AccordionItem>
        <AccordionItem  value="thaw">
            <AccordionHeader slot>
                "Thaw"
            </AccordionHeader>
            "An easy to use leptos component library"
        </AccordionItem>
    </Accordion>
}
```

### Accordion Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| open_items | `Model<HashSet<String>>` | `Default::default()` | Controls the state of the panel. |
| multiple | `bool` | `Default::default()` | Indicates if Accordion support multiple Panels opened at the same time. |
| collapsible | `bool` | `Default::default()` | Indicates if Accordion support multiple Panels closed at the same time. |
| children | `Children` |  |  |

### AccordionItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `MaybeSignal<String>` |  | Required value that identifies this item inside an Accordion component. |
| accordion_header | slot `AccordionHeader` |  |  |
| children | `Children` |  |  |

### AccordionHeader Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |
