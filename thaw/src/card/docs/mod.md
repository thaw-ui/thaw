# Card

```rust demo
view! {
    <Card>
        <CardHeader>
            <Body1>
                <b>"Header"</b>" 2022-02-22"
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Description"</Caption1>
            </CardHeaderDescription>
            <CardHeaderAction slot>
                <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
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

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |


### CardPreview Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### CardFooter Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
