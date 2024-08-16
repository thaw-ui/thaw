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
| class | `MaybeProp<String>` | `Default::default()` |  |
| value | `Model<String>` | `Default::default()` | Set the input value. |
| allow_value | `Option<ArcOneCallback<String, bool>>` | `None` | Check the incoming value, if it returns false, input will not be accepted. |
| input_type | `MaybeSignal<InputType>` | `InputType::Text` | An input can have different text-based types based on the type of value the user will enter. |
| placeholder | `MaybeProp<String>` | `Default::default()` | Placeholder text for the input. |
| disabled | `MaybeSignal<bool>` | `false` | Whether the input is disabled. |
| on_focus | `Option<BoxOneCallback<ev::FocusEvent>>` | `None` | Callback triggered when the input is focussed on. |
| on_blur | `Option<BoxOneCallback<ev::FocusEvent>>` | `None` | Callback triggered when the input is blurred. |
| parser | `OptionalProp<BoxOneCallback<String, Option<String>>>` | `None` | Modifies the user input before assigning it to the value. |
| format | `OptionalProp<BoxOneCallback<String, String>>` | `None` | Formats the value to be shown to the user |
| input_prefix | slot `Option<InputPrefix>` | `None` |  |
| input_suffix | slot `Option<InputSuffix>` | `None` |  |
| comp_ref | ref `ComponentRef<InputRef>` | `Default::default()` |  |

### InputPrefix Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### InputSuffix Props

| Name     | Type       | Default | Description |
| -------- | ---------- | ------- | ----------- |
| children | `Children` |         |             |

### InputRef Props

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |