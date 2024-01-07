# Drawer

```rust demo
let show = create_rw_signal(false);
let placement = create_rw_signal(DrawerPlacement::Top);

let open = Callback::new(move |new_placement: DrawerPlacement| {
    show.set(true);
    placement.set(new_placement);
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

### Drawer Props

| Name      | Type                           | Default                  | Desciption                                |
| --------- | ------------------------------ | ------------------------ | ----------------------------------------- |
| class     | `MaybeSignal<String>`          | `Default::default()`     | Addtional classes for the drawer element. |
| show      | `MaybeSignal<bool>`            |                          | Whether to show drawer.                   |
| title     | `MaybeSignal<String>`          | `Default::default()`     | Drawer title.                             |
| placement | `MaybeSignal<DrawerPlacement>` | `DrawerPlacement::Right` | Drawer placement.                         |
| width     | `MaybeSignal<String>`          | `520px`                  | Drawer width.                             |
| height    | `MaybeSignal<String>`          | `260px`                  | Drawer height.                            |
| children  | `Children`                     |                          | Drawer content.                           |
