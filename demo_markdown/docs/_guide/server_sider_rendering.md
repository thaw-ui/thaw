# Server Sider Rendering

To enable the ssr mode, the following configurations are required:

```toml
thaw = { ..., default-features = false, features = ["ssr"] }
```

To enable the hydrate mode, the following configurations are required:

```toml
thaw = { ..., default-features = false, features = ["hydrate"] }
```
