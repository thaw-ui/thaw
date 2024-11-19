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

### Resizable Columns

```rust demo
view! {
    <Table>
        <TableHeader>
            <TableRow>
                <TableHeaderCell resizable=true min_width=100.0 max_width=300.0>"Tag"</TableHeaderCell>
                <TableHeaderCell resizable=true>"Count"</TableHeaderCell>
                <TableHeaderCell>"Date"</TableHeaderCell>
            </TableRow>
        </TableHeader>
        <TableBody>
            <TableRow>
                <TableCell>
                    <TableCellLayout truncate=true>
                        "Renders content with overflow: hidden and text-overflow: ellipsis"
                    </TableCellLayout>
                </TableCell>
                <TableCell>
                    <TableCellLayout truncate=true>
                        "Renders content with overflow: hidden and text-overflow: ellipsis"
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

### Table & TableHeader & TableBody & TableRow & TableCell Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### TableHeaderCell Props

| Name      | Type                | Default              | Description                                                 |
| --------- | ------------------- | -------------------- | ----------------------------------------------------------- |
| class     | `MaybeProp<String>` | `Default::default()` |                                                             |
| resizable | `bool`              | `false`              | Whethe the column width can be dragged.                     |
| min_width | `Option<f64>`       | `None`               | Min width of the column. Only works when resizable is true. |
| max_width | `Option<f64>`       | `None`               | Max width of the column. Only works when resizable is true. |
| children  | `Children`          |                      |                                                             |

### TableCellLayout Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| truncate | `Signal<bool>` | `false` | Renders content with overflow: hidden and text-overflow: ellipsis |
| children | `Children` |  |  |
