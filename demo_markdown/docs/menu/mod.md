# Menu

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Menu value>
        <MenuItem key="a" label="and"/>
        <MenuItem key="o" label="or"/>
    </Menu>
}
```

### Menu Props

| Name     | Type                  | Default              | Description                             |
| -------- | --------------------- | -------------------- | --------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the menu element. |
| value    | `MaybeSignal<String>` | `Default::default()` | The selected item key of the menu.      |
| children | `Children`            |                      | Menu's content.                         |

### MenuGroup Props

| Name     | Type                  | Default              | Description                                   |
| -------- | --------------------- | -------------------- | --------------------------------------------- |
| class    | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the menu group element. |
| label    | `String`              |                      | The label of the menu group.                  |
| children | `Children`            |                      | MenuGroup's content.                          |

### MenuItem Props

| Name  | Type                  | Default              | Description                                  |
| ----- | --------------------- | -------------------- | -------------------------------------------- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the menu item element. |
| label | `MaybeSignal<String>` | `Default::default()` | The label of the menu item.                  |
| key   | `MaybeSignal<String>` | `Default::default()` | The indentifier of the menu item.            |
