# Tabbar

```rust
use thaw::mobile::*;

let value = create_rw_signal(String::from("o"));

view! {
    <div style="height: 100vh;">
        {move || value.get()}
        <Tabbar value>
            <TabbarItem key="a">"and"</TabbarItem>
            <TabbarItem key="i">"if"</TabbarItem>
            <TabbarItem key="o" icon=icondata::AiCloseOutlined>
                "or"
            </TabbarItem>
        </Tabbar>
    </div>
}
```

### Tabbar Props

| Name     | Type               | Default              | Description       |
| -------- | ------------------ | -------------------- | ----------------- |
| value    | `RwSignal<String>` | `Default::default()` | Tabbar's value.   |
| children | `Children`         |                      | Tabbar's content. |

### TabbarItem Props

| Name     | Type                  | Default | Description                         |
| -------- | --------------------- | ------- | ----------------------------------- |
| key      | `MaybeSignal<String>` |         | The indentifier of the tabbar item. |
| icon     | `Option<Icon>`        | `None`  | TabbarItem's icon.                  |
| children | `Children`            |         | TabbarItem's content.               |
