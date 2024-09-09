# Flex

```rust demo
view! {
    <Flex>
        <Button>"1"</Button>
        <Button>"2"</Button>
        <Button>"3"</Button>
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
            <Button style="height: 60px">"Start"</Button>
            <Button style="height: 40px">"Start"</Button>
            <Button>"Start"</Button>
        </Flex>
        <Flex align=FlexAlign::Center>
            <Button style="height: 60px">"Center"</Button>
            <Button style="height: 40px">"Center"</Button>
            <Button>"Center"</Button>
        </Flex>
        <Flex align=FlexAlign::End>
            <Button style="height: 60px">"End"</Button>
            <Button style="height: 40px">"End"</Button>
            <Button>"End"</Button>
        </Flex>
    </Flex>
}
```

### Justify

```rust demo
view! {
    <Flex vertical=true>
        <Flex justify=FlexJustify::SpaceAround>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
            <Button>"SpaceAround"</Button>
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

| Name     | Type                                | Default              | Description                              |
| -------- | ----------------------------------- | -------------------- | ---------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the space element. |
| vertical | `bool`                              | `false`              | Whether to lay out vertically.           |
| gap      | `FlexGap`                           | `FlexGap::Medium`    | Flex's gap.                              |
| align    | `OptionalMaybeSignal<FlexAlign>`    | `None`               | Vertical arrangement.                    |
| justify  | `OptionalMaybeSignal<FlexJustify>`  | `None`               | Horizontal arrangement.                  |
| children | `Children`                          |                      | Flex's content.                          |
