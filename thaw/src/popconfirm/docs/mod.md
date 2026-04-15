# Popconfirm

A popover that asks for confirmation before executing an action.

### Basic

```rust demo
let toaster = ToasterInjection::expect_context();

let on_confirm = Callback::new(move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Confirmed"</ToastTitle>
        </Toast>
    }, ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top));
});

let on_cancel = Callback::new(move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Cancelled"</ToastTitle>
        </Toast>
    }, ToastOptions::default().with_intent(ToastIntent::Warning).with_position(ToastPosition::Top));
});

view! {
    <Popconfirm
        title="Delete the task"
        description="Are you sure to delete this task?"
        on_confirm
        on_cancel
    >
        <Button>"Delete"</Button>
    </Popconfirm>
}
```

### Custom Button Text

```rust demo
let toaster = ToasterInjection::expect_context();

let on_confirm = Callback::new(move |_| {
    toaster.dispatch_toast(move || view! {
        <Toast>
            <ToastTitle>"Item removed"</ToastTitle>
        </Toast>
    }, ToastOptions::default().with_intent(ToastIntent::Success).with_position(ToastPosition::Top));
});

view! {
    <Popconfirm
        title="Remove item"
        description="This action cannot be undone."
        on_confirm
        ok_text="Confirm"
        cancel_text="Cancel"
    >
        <Button>"Remove"</Button>
    </Popconfirm>
}
```

### Position

```rust demo
let on_confirm = Callback::new(move |_| {});

view! {
    <Space>
        <Popconfirm title="Delete this?" on_confirm position=PopoverPosition::Top>
            <Button>"Top"</Button>
        </Popconfirm>
        <Popconfirm title="Delete this?" on_confirm position=PopoverPosition::Bottom>
            <Button>"Bottom"</Button>
        </Popconfirm>
        <Popconfirm title="Delete this?" on_confirm position=PopoverPosition::Right>
            <Button>"Right"</Button>
        </Popconfirm>
    </Space>
}
```

### Popconfirm Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| title | `String` | `""` | Title shown in the popover. |
| description | `String` | `""` | Description shown below the title. |
| position | `PopoverPosition` | `PopoverPosition::Top` | Configures the position of the popover. |
| appearance | `MaybeProp<PopoverAppearance>` | `Default::default()` | A popover can appear styled with brand or inverted. |
| trigger_type | `PopoverTriggerType` | `PopoverTriggerType::Click` | Action that displays the popconfirm. |
| size | `Signal<PopoverSize>` | `PopoverSize::Medium` | Size of the popover. |
| on_confirm | `Callback<()>` |  | Called when the user clicks the confirm button. |
| on_cancel | `Option<Callback<()>>` | `None` | Called when the user clicks the cancel button. |
| on_open | `Option<BoxCallback>` | `None` | Listen for popover open events. |
| on_close | `Option<BoxCallback>` | `None` | Listen for popover close events. |
| ok_text | `&'static str` | `"Yes"` | Text for the confirm button. |
| cancel_text | `&'static str` | `"No"` | Text for the cancel button. |
| children | `T: AddAnyAttr + IntoView + Send + 'static` |  | The trigger element. |
