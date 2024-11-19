# Input

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Textarea value placeholder="Textarea"/>
}
```

### Disabled

```rust demo
let value = RwSignal::new(String::from("o"));

view! {
    <Textarea value disabled=true/>
}
```

### Resize

```rust demo
view! {
    <Space vertical=true>
        <Textarea placeholder=r#"Textarea with resize set to "none""#/>
        <Textarea placeholder=r#"Textarea with resize set to "vertical""# resize=TextareaResize::Vertical/>
        <Textarea placeholder=r#"Textarea with resize set to "horizontal""# resize=TextareaResize::Horizontal/>
        <Textarea placeholder=r#"Textarea with resize set to "both""# resize=TextareaResize::Both/>
    </Space>
}
```

### Textarea Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| id | `MaybeProp<String>` | `Default::default()` |  |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| rules | `Vec<TextareaRule>` | `vec![]` | The rules to validate Field. |
| value | `Model<String>` | `Default::default()` | The value of the Textarea. |
| allow_value | `Option<BoxOneCallback<String, bool>>` | `None` | Check the incoming value, if it returns false, input will not be accepted. |
| placeholder | `MaybeProp<String>` | `Default::default()` | Placeholder text for the input. |
| disabled | `Signal<bool>` | `false` | Whether the input is disabled. |
| on_focus | `Option<BoxOneCallback<ev::FocusEvent>>` | `None` | Callback triggered when the input is focussed on. |
| on_blur | `Option<BoxOneCallback<ev::FocusEvent>>` | `None` | Callback triggered when the input is blurred. |
| resize | `Signal<TextareaResize>` | `TextareaResize::None` | Which direction the Textarea is allowed to be resized. |
| comp_ref | ref `ComponentRef<TextareaRef>` | `Default::default()` |  |

### TextareaRef Props

| Name  | Type        | Description              |
| ----- | ----------- | ------------------------ |
| focus | `Fn(&self)` | Focus the input element. |
| blur  | `Fn(&self)` | Blur the input element.  |
