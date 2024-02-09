# Collapse

```rust demo
use std::collections::HashSet;

let value = create_rw_signal(HashSet::from(["thaw".to_string()]));

view! {
    <Collapse value>
        <CollapseItem title="Leptos" key="leptos">
            "Build fast web applications with Rust."
        </CollapseItem>
        <CollapseItem title="Thaw" key="thaw">
            "An easy to use leptos component library"
        </CollapseItem>
    </Collapse>
}
```

### Accordion

Like an accordion.

```rust demo
view! {
    <Collapse accordion=true>
        <CollapseItem title="Leptos" key="leptos">
            "Build fast web applications with Rust."
        </CollapseItem>
        <CollapseItem title="Thaw" key="thaw">
            "An easy to use leptos component library."
        </CollapseItem>
        <CollapseItem title="Naive UI" key="naive-ui">
            "A Vue 3 Component Library. Fairly Complete. Theme Customizable. Uses TypeScript. Fast."
        </CollapseItem>
    </Collapse>
}
```

### Collapse Props

| Name      | Type                                | Default              | Description                                 |
| --------- | ----------------------------------- | -------------------- | ------------------------------------------- |
| class     | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the collapse element. |
| value     | `Model<HashSet<String>>`            | `Default::default()` | Currently active panel.                     |
| accordion | `bool`                              | `false`              | Only allow one panel open at a time.        |
| children  | `Children`                          |                      | Collapse's content.                         |

### CollapseItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the collapse item element. |
| title | `MaybeSignal<String>` |  | The title of the CollapseItem. |
| key | `MaybeSignal<String>` |  | The indentifier of CollapseItem. |
| chilren | `Children` |  | CollapseItem's content. |
