# Grid

```rust demo
use leptos_meta::Style;

view! {
    <Space vertical=true>
        <Grid>
            <GridItem>"123"</GridItem>
            <GridItem>"456"</GridItem>
            <GridItem>"789"</GridItem>
        </Grid>

        <Grid cols=2>
            <GridItem>"123"</GridItem>
            <GridItem>"456"</GridItem>
            <GridItem>"789"</GridItem>
        </Grid>
    </Space>
    <Style>
        ".thaw-grid-item {
            height: 60px;
            text-align: center;
            line-height: 60px;
        }
        .thaw-grid-item:nth-child(odd) {
            background-color: #0078ff88;
        }
        .thaw-grid-item:nth-child(even) {
            background-color: #0078ffaa;
        }"
    </Style>
}
```

### Gap

```rust demo
use leptos_meta::Style;

view! {
    <Grid cols=3 x_gap=8 y_gap=8>
        <GridItem>"123"</GridItem>
        <GridItem>"321"</GridItem>
        <GridItem>"123"</GridItem>
        <GridItem>"456"</GridItem>
        <GridItem>"7"</GridItem>
        <GridItem>"123"</GridItem>
        <GridItem>"123"</GridItem>
        <GridItem column=2>"1234"</GridItem>
        <GridItem>"567"</GridItem>
        <GridItem>"567"</GridItem>
    </Grid>
    <Style>
        ".thaw-grid-item {
            height: 60px;
            text-align: center;
            line-height: 60px;
        }
        .thaw-grid-item:nth-child(odd) {
            background-color: #0078ff88;
        }
        .thaw-grid-item:nth-child(even) {
            background-color: #0078ffaa;
        }"
    </Style>
}
```

### Offset

```rust demo
use leptos_meta::Style;

view! {
    <Grid cols=4>
        <GridItem offset=2>"123"</GridItem>
        <GridItem>"456"</GridItem>
    </Grid>
    <Style>
        ".thaw-grid-item {
            height: 60px;
            text-align: center;
            line-height: 60px;
        }
        .thaw-grid-item:nth-child(odd) {
            background-color: #0078ff88;
        }
        .thaw-grid-item:nth-child(even) {
            background-color: #0078ffaa;
        }"
    </Style>
}
```

### Grid Props

| Name     | Type                | Default              | Desciption                 |
| -------- | ------------------- | -------------------- | -------------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                            |
| cols     | `MaybeSignal<u16>`  | `1`                  | Number of grids displayed. |
| x_gap    | `MaybeSignal<u16>`  | `0`                  | Horizontal gap.            |
| y_gap    | `MaybeSignal<u16>`  | `0`                  | Vertical gap.              |
| children | `Children`          |                      |                            |

### GridItem Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| column | `MaybeSignal<u16>` | `1` | The number of columns occupied by the grid. The grid item would be hidden if it's 0. |
| offset | `MaybeSignal<u16>` | `0` | The number of intervals to the left of the grid. |
| children | `Children` |  |  |
