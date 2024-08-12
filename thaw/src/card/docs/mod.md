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

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### CardHeader Props

| Name                    | Type                                 | Default              | Description |
| ----------------------- | ------------------------------------ | -------------------- | ----------- |
| class                   | `MaybeProp<String>`                  | `Default::default()` |             |
| card_header_description | slot `Option<CardHeaderDescription>` | `None`               |             |
| card_header_action      | slot `Option<CardHeaderAction>`      | `None`               |             |
| children                | `Children`                           |                      |             |

### CardHeaderDescription Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### CardHeaderAction Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

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
