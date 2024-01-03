# Card

```rust demo
view! {
    <Space vertical=true>
        <Card title="title">"content"</Card>
        <Card title="title">
            <CardHeaderExtra slot>"header-extra"</CardHeaderExtra>
            "content"
        </Card>
        <Card title="title">
            <CardHeader slot>"header"</CardHeader>
            "content"
        </Card>
        <Card title="title">
            <CardHeaderExtra slot>"header-extra"</CardHeaderExtra>
            "content"
            <CardFooter slot>"footer"</CardFooter>
        </Card>
    </Space>
}
```

### Card Props

| Name     | Type                  | Default              | Description                             |
| -------- | --------------------- | -------------------- | --------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the card element. |
| title    | `MaybeSignal<String>` | `Default::default()` | Card title.                             |
| children | `Children`            |                      | Card's content.                         |

### Card Slots

| Name            | Default | Description           |
| --------------- | ------- | --------------------- |
| CardHeader      | `None`  | Header content.       |
| CardHeaderExtra | `None`  | Header extra content. |
| CardFooter      | `None`  | Footer content.       |
