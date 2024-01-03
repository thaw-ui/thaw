# Image

```rust demo
view! {
    <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
    <Image width="200px" height="200px"/>
}
```

### Image Props

| Name          | Type                  | Default              | Desciption                                     |
| ------------- | --------------------- | -------------------- | ---------------------------------------------- |
| class         | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the image element.       |
| src           | `MaybeSignal<String>` | `Default::default()` | Image source.                                  |
| alt           | `MaybeSignal<String>` | `Default::default()` | Image alt information.                         |
| width         | `MaybeSignal<String>` | `Default::default()` | Image width.                                   |
| height        | `MaybeSignal<String>` | `Default::default()` | Image height.                                  |
| border_radius | `MaybeSignal<String>` | `Default::default()` | Image border radius.                           |
| object_fit    | `MaybeSignal<String>` | `Default::default()` | Object-fit type of the image in the container. |
