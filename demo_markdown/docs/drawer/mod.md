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
        <Button on_click=Callback::new(move |_| leptos::Callable::call(&open, DrawerPlacement::Top))>"Top"</Button>
        <Button on_click=Callback::new(move |_| leptos::Callable::call(&open, DrawerPlacement::Right))>"Right"</Button>
        <Button on_click=Callback::new(move |_| leptos::Callable::call(&open, DrawerPlacement::Bottom))>"Bottom"</Button>
        <Button on_click=Callback::new(move |_| leptos::Callable::call(&open, DrawerPlacement::Left))>"Left"</Button>
    </ButtonGroup>
    <Drawer title="Title" show placement>
        "Hello"
    </Drawer>
}
```

### No Modal

```rust demo
let show = create_rw_signal(false);

view! {
    <Button on_click=move |_| show.update(|show| *show = !*show)>"Toggle"</Button>
    <Drawer title="Title" show modal_type=DrawerModalType::NonModal>
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

### Scroll content

```rust demo
let show = create_rw_signal(false);

view! {
    <Button on_click=move |_| show.set(true)>"Open"</Button>
    <Drawer show width="160px" title="Scroll content">
        r#"This being said, the world is moving in the direction opposite to Clarke's predictions. In 2001: A Space Odyssey, in the year of 2001, which has already passed, human beings have built magnificent cities in space, and established permanent colonies on the moon, and huge nuclear-powered spacecraft have sailed to Saturn. However, today, in 2018, the walk on the moon has become a distant memory.And the farthest reach of our manned space flights is just as long as the two-hour mileage of a high-speed train passing through my city. At the same time, information technology is developing at an unimaginable speed. With the entire world covered by the Internet, people have gradually lost their interest in space, as they find themselves increasingly comfortable in the space created by IT. Instead of an exploration of the real space, which is full of real difficulties, people now just prefer to experience virtual space through VR. Just like someone said, "You promised me an ocean of stars, but you actually gave me Facebook.""#
    </Drawer>
}
```

### Drawer Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the drawer element. |
| show | `Model<bool>` |  | Whether to show drawer. |
| mask_closeable | `MaybeSignal<bool>` | `true` | Whether to emit hide event when click mask. |
| modal_type | `DrawerModalType` | `DrawerModalType::Modal` |  |
| close_on_esc | `bool` | `true` | Whether to close drawer on Esc is pressed. |
| title | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Drawer title. |
| placement | `MaybeSignal<DrawerPlacement>` | `DrawerPlacement::Right` | Drawer placement. |
| width | `MaybeSignal<String>` | `520px` | Drawer width. |
| height | `MaybeSignal<String>` | `260px` | Drawer height. |
| z_index | `MaybeSignal<i16>` | `2000` | z-index of the drawer. |
| mount | `DrawerMount` | `DrawerMount::Body` | Container node of the drawer. |
| children | `Children` |  | Drawer content. |
