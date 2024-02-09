# Badge

```rust demo
let value = create_rw_signal(0);

view! {
    <Space>
        <Badge value max=10>
            <Avatar/>
        </Badge>
        <Badge variant=BadgeVariant::Success value max=10>
            <Avatar/>
        </Badge>
        <Badge variant=BadgeVariant::Warning value max=10>
            <Avatar/>
        </Badge>
        <Badge variant=BadgeVariant::Warning dot=true>
            <Avatar/>
        </Badge>
        <Button on_click=move |_| value.update(|v| *v += 1)>"+1"</Button>
        <Button on_click=move |_| {
            value
                .update(|v| {
                    if *v != 0 {
                        *v -= 1;
                    }
                })
        }>"-1"</Button>
        "value:"
        {move || value.get()}
    </Space>
}
```

### Badge Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the badge element. |
| value | `MaybeSignal<u32>` | `0` | Badge's value. |
| max | `MaybeSignal<u32>` | `u32::MAX` | The maximum number of the badge when its value overflows. |
| variant | `MaybeSignal<BadgeVariant>` | `BadgeVariant::Error` | Badge variant. |
| dot | `MaybeSignal<bool>` | `false` | Show badge as dot. |
| children | `Children` |  | Badge's content. |
