# Auto Complete

```rust demo
let value = RwSignal::new(String::new());
let options = Memo::<Vec<_>>::new(move |_| {
    let prefix = value
        .get()
        .split_once('@')
        .map_or(value.get(), |v| v.0.to_string());
    vec!["@gmail.com", "@163.com"]
        .into_iter()
        .map(|suffix| (format!("{prefix}{suffix}"), format!("{prefix}{suffix}")))
        .collect()
});

view! {
    <AutoComplete value placeholder="Email">
        <For
            each=move || options.get()
            key=|option| option.0.clone()
            let:option
        >
            <AutoCompleteOption value=option.0>
                {option.1}
            </AutoCompleteOption>
        </For>
    </AutoComplete>
}
```

### Disabled

```rust demo
view! {
    <AutoComplete placeholder="Email" disabled=true/>
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
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Model<String>` | `Default::default()` | Input of autocomplete. |
| placeholder | `MaybeProp<String>` | `Default::default()` | Autocomplete's placeholder. |
| disabled | `Signal<bool>` | `false` | Whether the input is disabled. |
| clear_after_select | `Signal<bool>` | `false` | Whether to clear after selection. |
| blur_after_select | `Signal<bool>` | `false` | Whether to blur after selection. |
| auto_complete_prefix | slot `Option<AutoCompletePrefix>` | `None` |  |
| auto_complete_suffix | slot `Option<AutoCompleteSuffix>` | `None` |  |
| comp_ref | ref `ComponentRef<AutoCompleteRef>` | `Default::default()` |  |
| children | `Option<Children>` | `None` |  |

### AutoCompleteOption Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| value    | `String`            |                      | Option ID.  |
| children | `Children`          |                      |             |

### AutoCompletePrefix Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### AutoCompleteSuffix Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### AutoCompleteRef Props

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
