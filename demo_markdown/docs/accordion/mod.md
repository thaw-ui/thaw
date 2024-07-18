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