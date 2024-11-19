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
| class | `MaybeProp<String>` | `Default::default()` |  |
| src | `MaybeProp<String>` | `Default::default()` | path to the image you want to display. |
| alt | `MaybeProp<String>` | `Default::default()` | description of the image, which isn't mandatory but is incredibly useful for accessibility. |
| width | `MaybeProp<String>` | `Default::default()` | Image width. |
| height | `MaybeProp<String>` | `Default::default()` | Image height. |
| shape | `Signal<ImageShape>` | `ImageShape::Square` | An image can appear square, circular, or rounded. |
| block | `Signal<bool>` | `Default::default()` | An image can take up the width of its container. |
| shadow | `Signal<bool>` | `Default::default()` | An image can appear elevated with shadow. |
| fit | `Signal<ImageFit>` | `ImageFit::Fill` | An image can set how it should be resized to fit its container. |
