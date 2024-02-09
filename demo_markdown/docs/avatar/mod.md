# Avatar

```rust demo
view! {
    <Space>
        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg"/>
        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" round=true/>
        <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" size=50/>
    </Space>
}
```

### Avatar Props

| Name  | Type                                | Default              | Description                               |
| ----- | ----------------------------------- | -------------------- | ----------------------------------------- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the avatar element. |
| src   | `Option<MaybeSignal<String>>`       | `Default::default()` | Avatar's image source.                    |
| round | `MaybeSignal<bool>`                 | `false`              | Whether to display a rounded avatar.      |
| size  | `MaybeSignal<u16>`                  | `30`                 | Avatar's size.                            |
