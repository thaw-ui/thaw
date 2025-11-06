# Flex

```rust demo
view! {
    <Flex>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Flex>
    <Flex>
        <For
            each=move || 0..4
            key=move |i| i.clone()
            let:i
        >
            <Button>{i}</Button>
        </For>
    </Flex>
}
```

### Vertical

```rust demo
view! {
    <Flex vertical=true>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Flex>
}
```

### Gap

```rust demo
view! {
    <Flex gap=FlexGap::Large>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Flex>
    <Flex gap=FlexGap::WH(36, 36)>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
    </Flex>
}
```

### Align

```rust demo
view! {
    <Flex vertical=true>
        <Flex align=FlexAlign::Start>
            <Button attr:style="height: 60px">"Start"</Button>
            <Button attr:style="height: 40px">"Start"</Button>
            <Button>"Start"</Button>
        </Flex>
        <Flex align=FlexAlign::Center>
            <Button attr:style="height: 60px">"Center"</Button>
            <Button attr:style="height: 40px">"Center"</Button>
            <Button>"Center"</Button>
        </Flex>
        <Flex align=FlexAlign::End>
            <Button attr:style="height: 60px">"End"</Button>
            <Button attr:style="height: 40px">"End"</Button>
            <Button>"End"</Button>
        </Flex>
    </Flex>
}
```

### Justify

```rust demo
view! {
    <Flex vertical=true>
        <Flex justify=FlexJustify::SpaceBetween>
            <Button>"SpaceBetween"</Button>
            <Button>"SpaceBetween"</Button>
            <Button>"SpaceBetween"</Button>
        </Flex>
        <Flex justify=FlexJustify::SpaceAround>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
        </Flex>
        <Flex justify=FlexJustify::SpaceEvenly>
            <Button>"SpaceEvenly"</Button>
            <Button>"SpaceEvenly"</Button>
            <Button>"SpaceEvenly"</Button>
        </Flex>
        <Flex justify=FlexJustify::Start>
            <Button>"Start"</Button>
            <Button>"Start"</Button>
            <Button>"Start"</Button>
        </Flex>
        <Flex justify=FlexJustify::Center>
            <Button>"Center"</Button>
            <Button>"Center"</Button>
            <Button>"Center"</Button>
        </Flex>
        <Flex justify=FlexJustify::End>
            <Button>"End"</Button>
            <Button>"End"</Button>
            <Button>"End"</Button>
        </Flex>
    </Flex>
}
```

### Flex Props

| Name     | Type                     | Default              | Description                            |
| -------- | ------------------------ | -------------------- | -------------------------------------- |
| class    | `MaybeProp<String>`      | `Default::default()` |                                        |
| style    | `MaybeProp<String>`      | `Default::default()` |                                        |
| inline   | `Signal<bool>`           | `false`              | Whether it's display is `inline-flex`. |
| vertical | `bool`                   | `false`              | Whether to lay out vertically.         |
| gap      | `FlexGap`                | `FlexGap::Medium`    | Flex's gap.                            |
| align    | `MaybeProp<FlexAlign>`   | `None`               | Vertical arrangement.                  |
| justify  | `MaybeProp<FlexJustify>` | `None`               | Horizontal arrangement.                |
| children | `Children`               |                      |                                        |
