# Input

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Space vertical=true>
        <Input value/>
        <Input value variant=InputVariant::Password placeholder="Password"/>
        <TextArea value placeholder="Textarea"/>
    </Space>
}
```

### Disabled

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Space vertical=true>
        <Input value disabled=true/>
        <TextArea value disabled=true/>
    </Space>
}
```

### Invalid

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Space vertical=true>
        <Input value invalid=true/>
        <TextArea value invalid=true/>
    </Space>
}
```

### Imperative handle

```rust demo
let value = create_rw_signal(String::from("o"));
let input_ref = create_component_ref::<InputRef>();

view! {
    <Space vertical=true>
        <Space>
            <Button on_click=move |_| input_ref.get_untracked().unwrap().focus()>
                "Focus"
            </Button>
            <Button on_click=move |_| input_ref.get_untracked().unwrap().blur()>
                "Blur"
            </Button>
        </Space>
        <Input value comp_ref=input_ref/>
    </Space>
}
```

### Input attrs

```rust demo
view! {
    <Space>
        <label for="demo-input-attrs">"Do you like cheese?"</label>
        <Input attr:id="demo-input-attrs"/>
    </Space>
}
```

## Prefix & Suffix

```rust demo
let value = create_rw_signal(String::from("o"));

view! {
    <Space vertical=true>
        <Input value>
            <InputPrefix slot>
                <Icon icon=icondata::AiUserOutlined/>
            </InputPrefix>
        </Input>
        <Input value>
            <InputSuffix slot>"$"</InputSuffix>
        </Input>
        <Input value>
            <InputSuffix slot>
                <Icon icon=icondata::AiGithubOutlined/>
            </InputSuffix>
        </Input>
    </Space>
}
```

### Input Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeSignal<String>` | `Default::default()` | Addtional classes for the input element. |
| value | `RwSignal<String>` | `Default::default()` | Set the input value. |
| variant | `MaybeSignal<InputVariant>` | `InputVariant::Text` | Input's variant. |
| placeholder | `MaybeSignal<String>` | `Default::default()` | Placeholder of input. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| invalid | `MaybeSignal<bool>` | `false` | Whether the input is invalid. |
| allow_value | `Option<Callback<String, bool>>` | `None` | Check the incoming value, if it returns false, input will not be accepted. |
| on_focus | `Option<Callback<ev::FocusEvent>>` | `None` | Callback triggered when the input is focussed on. |
| on_blur | `Option<Callback<ev::FocusEvent>>` | `None` | Callback triggered when the input is blurred. |
| attr: | `Vec<(&'static str, Attribute)>` | `Default::default()` | The dom attrs of the input element inside the component. |

### Input Slots

| Name        | Default | Description          |
| ----------- | ------- | -------------------- |
| InputPrefix | `None`  | InputPrefix content. |
| InputSuffix | `None`  | InputSuffix content. |

### Input Ref

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |

### TextArea Props

Removes variant and slot from Input component.
