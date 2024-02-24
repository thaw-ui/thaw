# Drawer

```rust demo
let show = create_rw_signal(false);
let placement = create_rw_signal(DrawerPlacement::Top);

let open = Callback::new(move |new_placement: DrawerPlacement| {
    // Note: Since `show` changes are made in real time,
    // please put it in front of `show.set(true)` when changing `placement`.
    placement.set(new_placement);
    show.set(true);
});

view! {
    <ButtonGroup>
        <Button on_click=Callback::new(move |_| open.call(DrawerPlacement::Top))>"Top"</Button>
        <Button on_click=Callback::new(move |_| open.call(DrawerPlacement::Right))>"Right"</Button>
        <Button on_click=Callback::new(move |_| open.call(DrawerPlacement::Bottom))>"Bottom"</Button>
        <Button on_click=Callback::new(move |_| open.call(DrawerPlacement::Left))>"Left"</Button>
    </ButtonGroup>
    <Drawer title="Title" show placement>
        "Hello"
    </Drawer>
}
```

### Customize display area

```rust demo
let show = create_rw_signal(false);

view! {
    <div style="position: relative; overflow: hidden; height: 200px; background-color: #0078ff88;">
        <Button on_click=move |_| show.set(true)>"Open"</Button>
        <Drawer show mount=DrawerMount::None width="50%">
            "Current position"
        </Drawer>
    </div>
}
```

### Drawer Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the drawer element. |
| show | `Model<bool>` |  | Whether to show drawer. |
| title | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Drawer title. |
| placement | `MaybeSignal<DrawerPlacement>` | `DrawerPlacement::Right` | Drawer placement. |
| width | `MaybeSignal<String>` | `520px` | Drawer width. |
| height | `MaybeSignal<String>` | `260px` | Drawer height. |
| z_index | `MaybeSignal<i16>` | `2000` | z-index of the drawer. |
| mount | `DrawerMount` | `DrawerMount::Body` | Container node of the drawer. |
| children | `Children` |  | Drawer content. |
