# Drawer

```rust demo
let open = RwSignal::new(false);
let position = RwSignal::new(DrawerPosition::Top);

let open_f = move |new_position: DrawerPosition| {
    // Note: Since `show` changes are made in real time,
    // please put it in front of `show.set(true)` when changing `placement`.
    position.set(new_position);
    open.set(true);
};

view! {
    <ButtonGroup>
        <Button on_click=move |_| open_f(DrawerPosition::Top)>"Top"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Right)>"Right"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Bottom)>"Bottom"</Button>
        <Button on_click=move |_| open_f(DrawerPosition::Left)>"Left"</Button>
    </ButtonGroup>
    <OverlayDrawer open position>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
                 <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open.set(false)
                >
                    "x"
                </Button>
            </DrawerHeaderTitleAction>
            "Default Drawer"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <p>"Drawer content"</p>
        </DrawerBody>
    </OverlayDrawer>
}
```

### Overlay No Modal

```rust demo
let open = RwSignal::new(false);

view! {
    <Button on_click=move |_| open.update(|open| *open = !*open)>"Toggle"</Button>
    <Drawer open modal_type=DrawerModalType::NonModal>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
                 <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open.set(false)
                >
                    "x"
                </Button>
            </DrawerHeaderTitleAction>
            "Default Drawer"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <p>"Drawer content"</p>
        </DrawerBody>
    </Drawer>
}
```

### Inline

```rust demo
let open_left = RwSignal::new(false);
let open_right = RwSignal::new(false);
let open_buttom = RwSignal::new(false);

view! {
    <div style="display: flex; flex-direction: column; overflow: hidden; height: 400px; background-color: #0078ff88;">
        <div style="display: flex; overflow: hidden; height: 400px;">
            <InlineDrawer open=open_left>
                <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open_left.set(false)
                        >
                            "x"
                        </Button>
                    </DrawerHeaderTitleAction>
                    "Inline Drawer"
                </DrawerHeaderTitle>
                </DrawerHeader>
                <DrawerBody>
                    <p>"Drawer content"</p>
                </DrawerBody>
            </InlineDrawer>
            <div style="flex: 1">
                <Button on_click=move |_| open_left.set(true)>"Open left"</Button>
                <Button on_click=move |_| open_right.set(true)>"Open right"</Button>
                <Button on_click=move |_| open_buttom.set(true)>"Open buttom"</Button>
            </div>
            <InlineDrawer open=open_right position=DrawerPosition::Right>
                <DrawerHeader>
                <DrawerHeaderTitle>
                    <DrawerHeaderTitleAction slot>
                        <Button
                            appearance=ButtonAppearance::Subtle
                            on_click=move |_| open_right.set(false)
                        >
                            "x"
                        </Button>
                    </DrawerHeaderTitleAction>
                    "Inline Drawer"
                </DrawerHeaderTitle>
                </DrawerHeader>
                <DrawerBody>
                    <p>"Drawer content"</p>
                </DrawerBody>
            </InlineDrawer>
        </div>
        <InlineDrawer open=open_buttom position=DrawerPosition::Bottom>
            <DrawerHeader>
            <DrawerHeaderTitle>
                <DrawerHeaderTitleAction slot>
                    <Button
                        appearance=ButtonAppearance::Subtle
                        on_click=move |_| open_buttom.set(false)
                    >
                        "x"
                    </Button>
                </DrawerHeaderTitleAction>
                "Inline Drawer"
            </DrawerHeaderTitle>
            </DrawerHeader>
            <DrawerBody>
                <p>"Drawer content"</p>
            </DrawerBody>
        </InlineDrawer>
    </div>
}
```

### With Scroll

```rust demo
let open = RwSignal::new(false);

view! {
    <Button on_click=move |_| open.set(true)>"Open"</Button>
    <OverlayDrawer open>
        <DrawerHeader>
          <DrawerHeaderTitle>
            <DrawerHeaderTitleAction slot>
                 <Button
                    appearance=ButtonAppearance::Subtle
                    on_click=move |_| open.set(false)
                >
                    "x"
                </Button>
            </DrawerHeaderTitleAction>
            "Default Drawer"
          </DrawerHeaderTitle>
        </DrawerHeader>
        <DrawerBody>
          <p style="line-height: 40px">r#"This being said, the world is moving in the direction opposite to Clarke's predictions. In 2001: A Space Odyssey, in the year of 2001, which has already passed, human beings have built magnificent cities in space, and established permanent colonies on the moon, and huge nuclear-powered spacecraft have sailed to Saturn. However, today, in 2018, the walk on the moon has become a distant memory.And the farthest reach of our manned space flights is just as long as the two-hour mileage of a high-speed train passing through my city. At the same time, information technology is developing at an unimaginable speed. With the entire world covered by the Internet, people have gradually lost their interest in space, as they find themselves increasingly comfortable in the space created by IT. Instead of an exploration of the real space, which is full of real difficulties, people now just prefer to experience virtual space through VR. Just like someone said, "You promised me an ocean of stars, but you actually gave me Facebook.""#</p>
        </DrawerBody>
    </OverlayDrawer>
}
```

### OverlayDrawer Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| open | `Model<bool>` |  | Controls the open state of the Drawer. |
| mask_closeable | `MaybeSignal<bool>` | `true` | Whether to emit hide event when click mask. |
| close_on_esc | `bool` | `false` | Whether to close drawer on Esc is pressed. |
| position | `MaybeSignal<DrawerPosition>` | `DrawerPlacement::Left` | Position of the drawer. |
| size | `MaybeSignal<DrawerSize>` | `DrawerSize::Small` | Size of the drawer. |
| model_type | `DrawerModalType` | `DrawerModalType::Modal` | Dialog variations. |
| children | `Children` |  |  |

### InlineDrawer Props

| Name     | Type                          | Default                 | Desciption                             |
| -------- | ----------------------------- | ----------------------- | -------------------------------------- |
| class    | `MaybeProp<String>`           | `Default::default()`    |                                        |
| open     | `Model<bool>`                 |                         | Controls the open state of the Drawer. |
| position | `MaybeSignal<DrawerPosition>` | `DrawerPlacement::Left` | Position of the drawer.                |
| size     | `MaybeSignal<DrawerSize>`     | `DrawerSize::Small`     | Size of the drawer.                    |
| children | `Children`                    |                         |                                        |

### DrawerHeader Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### DrawerHeaderTitle Props

| Name                       | Type                                   | Default              | Description |
| -------------------------- | -------------------------------------- | -------------------- | ----------- |
| class                      | `MaybeProp<String>`                    | `Default::default()` |             |
| drawer_header_title_action | slot `Option<DrawerHeaderTitleAction>` | `None`               |             |
| children                   | `Children`                             |                      |             |

### DrawerHeaderTitleAction Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### DrawerBody Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
