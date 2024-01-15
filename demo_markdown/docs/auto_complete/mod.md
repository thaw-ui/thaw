# Auto Complete

```rust demo
let value = create_rw_signal(String::new());
let options = create_memo(move |_| {
    let prefix = value
        .get()
        .split_once('@')
        .map_or(value.get(), |v| v.0.to_string());
    vec!["@gmail.com", "@163.com"]
        .into_iter()
        .map(|suffix| AutoCompleteOption {
            label: format!("{prefix}{suffix}"),
            value: format!("{prefix}{suffix}"),
        })
        .collect()
});

view! {
    <AutoComplete value options placeholder="Email"/>
}
```

### Disabled

```rust demo
view! {
    <AutoComplete placeholder="Email" disabled=true/>
}
```

### Invalid

```rust demo
view! {
    <AutoComplete placeholder="Email" invalid=true/>
}
```

### Prefix & Suffix

```rust demo
view! {
    <Space vertical=true>
        <AutoComplete>
            <AutoCompletePrefix slot>
                <Icon icon=icondata::AiUserOutlined/>
            </AutoCompletePrefix>
        </AutoComplete>
        <AutoComplete>
            <AutoCompleteSuffix slot>
                <Icon icon=icondata::AiGithubOutlined/>
            </AutoCompleteSuffix>
        </AutoComplete>
    </Space>
}
```

### AutoComplete Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeSignal<String>` | `Default::default()` | Additional classes for the autocomplete element. |
| value | `RwSignal<String>` | `Default::default()` | Input of autocomplete. |
| placeholder | `RwSignal<String>` | `Default::default()` | Autocomplete's placeholder. |
| options | `MaybeSignal<Vec<AutoCompleteOption>>` | `Default::default()` | Options to autocomplete from. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| invalid | `MaybeSignal<bool>` | `false` | Whether the input is invalid. |
| clear_after_select | `MaybeSignal<bool>` | `false` | Whether to clear after selection. |
| on_select | `Option<Callback<String>>` | `None` | On select callback function. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |

### AutoCompleteOption Properties

| Name  | Type     | Description         |
| ----- | -------- | ------------------- |
| value | `String` | Option ID.          |
| label | `String` | Option label value. |

### AutoCompleteOption Slots

| Name               | Default | Description                 |
| ------------------ | ------- | --------------------------- |
| AutoCompletePrefix | `None`  | AutoCompletePrefix content. |
| AutoCompleteSuffix | `None`  | AutoCompleteSuffix content. |

### AutoComplete Ref

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
