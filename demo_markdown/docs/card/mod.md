# Card

```rust demo
view! {
    <Card>
        <CardHeader>
            <Body1>"Header"</Body1>
            <CardHeaderDescription slot>
                <Caption1>"Description"</Caption1>
            </CardHeaderDescription>
            <CardHeaderAction slot>
                <Button appearance=ButtonAppearance::Transparent>
                    "..."
                </Button>
            </CardHeaderAction>
        </CardHeader>
        <CardPreview>
            <img src="https://s3.bmp.ovh/imgs/2021/10/2c3b013418d55659.jpg" style="width: 100%"/>
        </CardPreview>
        <CardFooter>
            <Button>"Reply"</Button>
            <Button>"Share"</Button>
        </CardFooter>
    </Card>
}
```

### Card Props

| Name     | Type                                | Default              | Description                             |
| -------- | ----------------------------------- | -------------------- | --------------------------------------- |
| class    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the card element. |
| title    | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Card title.                             |
| children | `Children`                          |                      | Card's content.                         |

### Card Slots

| Name            | Default | Description           |
| --------------- | ------- | --------------------- |
| CardHeader      | `None`  | Header content.       |
| CardHeaderExtra | `None`  | Header extra content. |
| CardFooter      | `None`  | Footer content.       |
