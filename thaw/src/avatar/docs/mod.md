# Avatar

```rust demo
view! {
    <Avatar />
}
```

### Name

```rust demo
view! {
    <Avatar name="Ashley McCarthy" />
}
```

### Image

```rust demo
view! {
    <Avatar src="https://s3.bmp.ovh/imgs/2021/10/723d457d627fe706.jpg" />
}
```

### Shape

```rust demo
view! {
    <Avatar shape=AvatarShape::Square />
}
```

### Size

```rust demo
view! {
    <Space>
        <Avatar initials="16" size=16 />
        <Avatar initials="20" size=20 />
        <Avatar initials="24" size=24 />
        <Avatar initials="28" size=28 />
        <Avatar initials="32" size=32 />
        <Avatar initials="36" size=36 />
        <Avatar initials="40" size=40 />
        <Avatar initials="48" size=48 />
        <Avatar initials="56" size=56 />
        <Avatar initials="64" size=64 />
        <Avatar initials="72" size=72 />
        <Avatar initials="96" size=96 />
        <Avatar initials="120" size=120 />
        <Avatar initials="128" size=128 />
    </Space>
}
```

### Avatar Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| src | `MaybeProp<String>` | `Default::default()` | The Avatar's image. |
| name | `MaybeProp<String>` | `Default::default()` | The name of the person or entity represented by this Avatar. |
| initials | `MaybeProp<String>` | `Default::default()` | Custom initials. |
| shape | `MaybeSignal<AvatarShape>` | `Default::default()` | The avatar can have a circular or square shape. |
| size | `MaybeProp<u8>` | `Default::default()` | Size of the avatar in pixels. |
