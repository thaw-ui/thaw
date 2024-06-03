# Image

```rust demo
view! {
    <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="500px"/>
    <Image width="200px" height="200px"/>
}
```

### Shape

```rust demo
view! {
    <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="200px" height="200px"/>
    <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="200px" height="200px" shape=ImageShape::Circular/>
    <Image src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" width="200px" height="200px" shape=ImageShape::Rounded/>
}
```


### Image Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the image element. |
| src | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Image source. |
| alt | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Image alt information. |
| width | `MaybeSignal<String>` | `Default::default()` | Image width. |
| height | `MaybeSignal<String>` | `Default::default()` | Image height. |
| border_radius | `MaybeSignal<String>` | `Default::default()` | Image border radius. |
| object_fit | `MaybeSignal<String>` | `Default::default()` | Object-fit type of the image in the container. |
