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