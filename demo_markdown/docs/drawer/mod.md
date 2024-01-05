# Drawer

```rust demo
let show = create_rw_signal(false);
let placement = create_rw_signal(DrawerPlacement::Top);

view! {
    <Button on_click=move |_| show.set(true)>"Top"</Button>
    <Drawer title="title" show placement>
        "hello"
    </Drawer>
}
```