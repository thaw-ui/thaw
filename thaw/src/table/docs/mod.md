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

### Table & TableHeader & TableHeaderCell & TableBody & TableRow & TableCell & TableCellLayout Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
