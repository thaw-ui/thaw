# Table

```rust demo
view! {
    <Table>
        <thead>
            <tr>
                <th>"tag"</th>
                <th>"count"</th>
                <th>"date"</th>
            </tr>
        </thead>
        <tbody>
            <tr>
                <td>"div"</td>
                <td>"2"</td>
                <td>"2023-10-08"</td>
            </tr>
            <tr>
                <td>"span"</td>
                <td>"2"</td>
                <td>"2023-10-08"</td>
            </tr>
        </tbody>
    </Table>
}
```

### Table Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the table element. |
| style | `MaybeSignal<String>` | `Default::default()` | Table's style. |
| single_row | `MaybeSignal<bool>` | `true` | Whether columns are not divided. If the prop is true, table cell has no border-right. |
| single_column | `MaybeSignal<bool>` | `false` | Whether rows are not divided. If the prop is true, table cell has no border-bottom. |
| children | `Children` |  | Table's content. |
