# Tree

```rust demo
view! {
    <Tree>
        <TreeItem item_type=TreeItemType::Branch>
            <TreeItemLayout>"level 1, item 1"</TreeItemLayout>
            <Tree>
                <TreeItem item_type=TreeItemType::Leaf>
                    <TreeItemLayout>"level 2, item 1"</TreeItemLayout>
                </TreeItem>
                <TreeItem item_type=TreeItemType::Leaf>
                    <TreeItemLayout>"level 2, item 2"</TreeItemLayout>
                </TreeItem>
                <TreeItem item_type=TreeItemType::Leaf>
                    <TreeItemLayout>"level 2, item 3"</TreeItemLayout>
                </TreeItem>
            </Tree>
        </TreeItem>
        <TreeItem item_type=TreeItemType::Branch>
            <TreeItemLayout>"level 1, item 2"</TreeItemLayout>
            <Tree>
                <TreeItem item_type=TreeItemType::Branch>
                    <TreeItemLayout>"level 2, item 1"</TreeItemLayout>
                    <Tree>
                        <TreeItem item_type=TreeItemType::Leaf>
                            <TreeItemLayout>"level 3, item 1"</TreeItemLayout>
                        </TreeItem>
                    </Tree>
                </TreeItem>
            </Tree>
        </TreeItem>
        <TreeItem item_type=TreeItemType::Leaf>
            <TreeItemLayout>"level 1, item 3"</TreeItemLayout>
        </TreeItem>
    </Tree>
}
```

### Size

A tree can be displayed in a small or medium (default) size.

```rust demo
view! {
    <Tree size=TreeSize::Small>
        <TreeItem item_type=TreeItemType::Branch>
            <TreeItemLayout>"level 1, item 2"</TreeItemLayout>
            <Tree>
                <TreeItem item_type=TreeItemType::Branch>
                    <TreeItemLayout>"level 2, item 1"</TreeItemLayout>
                    <Tree>
                        <TreeItem item_type=TreeItemType::Leaf>
                            <TreeItemLayout>"level 3, item 1"</TreeItemLayout>
                        </TreeItem>
                    </Tree>
                </TreeItem>
            </Tree>
        </TreeItem>
    </Tree>
    <Tree>
        <TreeItem item_type=TreeItemType::Branch>
            <TreeItemLayout>"level 1, item 2"</TreeItemLayout>
            <Tree>
                <TreeItem item_type=TreeItemType::Branch>
                    <TreeItemLayout>"level 2, item 1"</TreeItemLayout>
                    <Tree>
                        <TreeItem item_type=TreeItemType::Leaf>
                            <TreeItemLayout>"level 3, item 1"</TreeItemLayout>
                        </TreeItem>
                    </Tree>
                </TreeItem>
            </Tree>
        </TreeItem>
    </Tree>
}
```

### Tree Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| open_items | `Model<HashSet<String>>` | `HashSet::default()` | This refers to a list of ids of opened tree items. Controls the state of the open tree items. These property is ignored for subtrees. |
| size | `Signal<TreeSize>` | `TreeSize::Medium` | Size of the tree item. |
| children | `Children` |  |  |

### TreeItem Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| item_type | `TreeItemType` | `TreeItemType::Leaf` | A tree item can be a leaf or a branch. |
| value | `Option<String>` | `None` | A tree item should have a well defined value, in case one is not provided by the user by this prop one will be inferred internally. |
| children | `Children` |  |  |

### TreeItemLayout Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |
