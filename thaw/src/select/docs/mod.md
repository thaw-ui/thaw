# Select

```rust demo
view! {
    <Select>
        <option>"Red"</option>
        <option>"Green"</option>
        <option>"Blue"</option>
    </Select>
}
```

### Controlled

```rust demo
let value = RwSignal::new("Red".to_string());

view! {
    <Select value>
        <option>"Red"</option>
        <option>"Green"</option>
        <option>"Blue"</option>
    </Select>
    <Button on_click=move |_| value.set("Blue".to_string())>
        "Select Blue"
    </Button>
}
```

### Disabled

```rust demo
view! {
    <Select disabled=true>
        <option>"Red"</option>
        <option>"Green"</option>
        <option>"Blue"</option>
    </Select>
}
```

### Initial Value

```rust demo
view! {
    <Select default_value="Green">
        <option>"Red"</option>
        <option>"Green"</option>
        <option>"Blue"</option>
    </Select>
}
```

### Select Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<SelectRule>` | `vec![]` | The rules to validate Field. |
| value | `Model<String>` | `Default::default()` | Set the select value. |
| default_value | `Option<String>` | `None` |  |
| disabled | `MaybeSignal<bool>` | `false` | Whether the select is disabled. |
| children | `Children` |  |  |
