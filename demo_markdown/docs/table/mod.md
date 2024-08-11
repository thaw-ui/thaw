# Table

```rust demo
view! {
    <Table>
        <TableHeader>
            <TableRow>
                <TableHeaderCell>"Tag"</TableHeaderCell>
                <TableHeaderCell>"Count"</TableHeaderCell>
                <TableHeaderCell>"Date"</TableHeaderCell>
            </TableRow>
        </TableHeader>
        <TableBody>
            <TableRow>
                <TableCell>
                    <TableCellLayout>
                        "div"
                    </TableCellLayout>
                </TableCell>
                <TableCell>
                    <TableCellLayout>
                        "2"
                    </TableCellLayout>
                </TableCell>
                <TableCell>
                    <TableCellLayout>
                        "2023-10-08"
                    </TableCellLayout>
                </TableCell>
            </TableRow>
            <TableRow>
                <TableCell>"span"</TableCell>
                <TableCell>"2"</TableCell>
                <TableCell>"2023-10-08"</TableCell>
            </TableRow>
        </TableBody>
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
