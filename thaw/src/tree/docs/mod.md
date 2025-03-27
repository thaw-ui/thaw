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
