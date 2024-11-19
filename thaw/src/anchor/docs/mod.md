# Anchor

```rust demo
view! {
    <Anchor>
        <AnchorLink title="Web API" href="#web">
            <AnchorLink title="DOM" href="#dom"/>
            <AnchorLink title="SVG" href="#svg"/>
            <AnchorLink title="File API" href="#file"/>
        </AnchorLink>
        <AnchorLink title="Rust" href="#rust"/>
        <AnchorLink title="Anchor Props" href="#anchor-props"/>
        <AnchorLink title="AnchorLink Props" href="#anchorlink-props"/>
    </Anchor>
}
```

```rust demo
view! {
    <Anchor>
        <AnchorLink title="Anchor" href="#anchor"/>
        <AnchorLink title="Anchor Props" href="#anchor-props"/>
        <AnchorLink title="AnchorLink Props" href="#anchorlink-props"/>
    </Anchor>
}
```

### Anchor Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| offset_target | `Option<OffsetTarget>` | `None` | The element or selector used to calc offset of link elements. If you are not scrolling the entire document but only a part of it, you may need to set this. |
| children | `Children` |  |  |

### AnchorLink Props

| Name     | Type                | Default              | Description          |
| -------- | ------------------- | -------------------- | -------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                      |
| title    | `Signal<String>`    |                      | The content of link. |
| href     | `String`            |                      | The target of link.  |
| children | `Option<Children>`  | `None`               |                      |
