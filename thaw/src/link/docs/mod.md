# Link

```rust demo
view! {
    <Space>
        <Link href="http://example.com">
            "This is a link"
        </Link>
        <Link>
            "This is a link"
        </Link>
        <Link span=true>
            "This is a link"
        </Link>
    </Space>
}
```

### Inline

```rust demo
view! {
    <div>
        "This is an "
        <Link href="http://example.com" inline=true>
            "inline link"
        </Link>
        " used alongside other text."
    </div>
}
```

### Disabled

```rust demo
view! {
    <Space>
        <Link href="http://example.com" disabled=true>
            "This is a link"
        </Link>
        <Link disabled=true>
            "This is a link"
        </Link>
        <Link span=true disabled=true>
            "This is a link"
        </Link>
    </Space>
}
```

### Disabled Focusable

```rust demo
view! {
    <Space>
        <Link href="http://example.com" disabled_focusable=true>
            "This is a link"
        </Link>
        <Link disabled_focusable=true>
            "This is a link"
        </Link>
        <Link span=true disabled_focusable=true>
            "This is a link"
        </Link>
    </Space>
}
```

### Link Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| span | `bool` | `false` |  |
| href | `Option<Signal<String>>` | `None` |  |
| inline | `Signal<bool>` | `false` | If true, changes styling when the link is being used alongside other text content. |
| disabled | `Signal<bool>` | `false` | Whether the link is disabled. |
| disabled_focusable | `Signal<bool>` | `false` | When set, allows the link to be focusable even when it has been disabled. |
| children | `Children` |  |  |
