# Input

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Space vertical=true>
        <Input value/>
        <Input value input_type=InputType::Password placeholder="Password"/>
    </Space>
}
```

## Prefix & Suffix

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Space vertical=true>
        <Input value>
            <InputPrefix slot>
                <Icon icon=icondata::AiUserOutlined/>
            </InputPrefix>
        </Input>
        <Input value>
            <InputSuffix slot>
                <Icon icon=icondata::AiGithubOutlined/>
            </InputSuffix>
        </Input>
        <Input value>
            <InputPrefix slot>"$"</InputPrefix>
            <InputSuffix slot>".00"</InputSuffix>
        </Input>
    </Space>
}
```

### Disabled

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Input value disabled=true/>
}
```

### Placeholder

```rust demo
view! {
    <Input placeholder="This is a placeholder"/>
}
```

### Imperative handle

```rust demo
let value = RwSignal::new(String::from("o"));
let input_ref = ComponentRef::<InputRef>::new();

let focus = move |_| {
    input_ref.get_untracked().unwrap().focus()
};

let blur = move |_| {
    input_ref.get_untracked().unwrap().blur()
};

view! {
    <Space vertical=true>
        <Space>
            <Button on_click=focus>
                "Focus"
            </Button>
            <Button on_click=blur>
                "Blur"
            </Button>
        </Space>
        <Input value comp_ref=input_ref/>
    </Space>
}
```

### Custom parsing

```rust demo
let value = RwSignal::new(String::from("loren_ipsun"));

let format = move |v: String| {
    v.replace("_", " ")
};
let parser = move |v: String| {
    Some(v.replace(" ", "_"))
};

view! {
    <Input value parser format />
    <p>"Underlying value: "{ value }</p>
}
```

### Input Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Addtional classes for the input element. |
| value | `Model<String>` | `Default::default()` | Set the input value. |
| variant | `MaybeSignal<InputVariant>` | `InputVariant::Text` | Input's variant. |
| placeholder | `OptionalProp<MaybeSignal<String>>` | `Default::default()` | Placeholder of input. |
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
