# Server Sider Rendering

To enable the ssr mode, the following configurations are required:

```toml
thaw = { ..., features = ["ssr"] }
```

To enable the hydrate mode, the following configurations are required:

```toml
thaw = { ..., features = ["hydrate"] }
```

### cargo-leptos

If you use [cargo-leptos](https://github.com/leptos-rs/cargo-leptos), Remember to add thaw to your `Cargo.toml` file in the corresponding feature, e.g.

```toml
[features]
...
hydrate = [..., "thaw/hydrate"]
ssr = [
  ...,
  "thaw/ssr",
]
```
