# CardHeader

```rust demo
view!{
    <Flex vertical=true>
        <CardHeader>
            <Body1>
                <b>"App Name"</b>
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Developer"</Caption1>
            </CardHeaderDescription>
            <CardHeaderAction slot>
                <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
            </CardHeaderAction>
        </CardHeader>
        <CardHeader>
            <Body1>
                <b>"App Name"</b>
            </Body1>
            <CardHeaderAction slot>
                <Button appearance=ButtonAppearance::Transparent icon=icondata::AiMoreOutlined />
            </CardHeaderAction>
        </CardHeader>
        <CardHeader>
            <Body1>
                <b>"App Name"</b>
            </Body1>
            <CardHeaderDescription slot>
                <Caption1>"Developer"</Caption1>
            </CardHeaderDescription>
        </CardHeader>
        <CardHeader>
            <Body1>
                <b>"App Name"</b>
            </Body1>
        </CardHeader>
    </Flex>
}
```

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