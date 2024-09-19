# Field

```rust demo
view! {
    <Field label="Example field">
        <Input />
    </Field>
}
```

### Horizontal Orientation

```rust demo
view! {
    <Field
        label="Horizontal"
        orientation=FieldOrientation::Horizontal
    >
        <Input />
    </Field>
}
```

### Validation rules

```rust demo
view! {
    <form>
        <FieldContextProvider>
            <Field label="Username" name="username">
                <Input rules=vec![InputRule::required(true.into())]/>
            </Field>
            <Field label="Password" name="password">
                <Input input_type=InputType::Password rules=vec![InputRule::required(true.into())]/>
            </Field>
            <Field name="remember">
                <CheckboxGroup rules=vec![CheckboxGroupRule::required(true.into())] >
                    <Checkbox label="Remember me" value="true"/>
                </CheckboxGroup>
            </Field>
            <Field label="Textarea" name="textarea">
                <Textarea rules=vec![TextareaRule::required(true.into())]/>
            </Field>
            <Field label="Radio" name="radio">
                <RadioGroup rules=vec![RadioGroupRule::required(true.into())] >
                    <Radio label="0" value="false"/>
                    <Radio label="1" value="true"/>
                </RadioGroup>
            </Field>
            <Field label="Combobox" name="combobox">
                <Combobox rules=vec![ComboboxRule::required(true.into())] placeholder="Select an animal" clearable=true>
                    <ComboboxOption value="cat" text="Cat"/>
                    <ComboboxOption value="dog" text="Dog" />
                </Combobox>
            </Field>
            <Field label="Select" name="select">
                <Select rules=vec![SelectRule::required(true.into())] >
                    <option>"Red"</option>
                    <option>"Green"</option>
                    <option>"Blue"</option>
                </Select>
            </Field>
            <Field label="SpinButton" name="spinbutton">
                <SpinButton
                    step_page=1
                    rules=vec![SpinButtonRule::validator(move |v, _| {
                        if v % 2 == 0 {
                            Err(FieldValidationState::Error("It has to be odd!".to_string()))
                        } else {
                            Ok(())
                        }
                    })]
                />
            </Field>
            <Field label="Date" name="date">
                <DatePicker rules=vec![DatePickerRule::required(true.into())]/>
            </Field>
            <Field label="Time" name="time">
                <TimePicker rules=vec![TimePickerRule::required(true.into())]/>
            </Field>
            <Field label="Slider" name="slider">
                <Slider
                    step=25.0
                    rules=vec![SliderRule::validator(move |v, _| {
                        if v % 2.0 == 0.0 {
                            Err(FieldValidationState::Error("It has to be odd!".to_string()))
                        } else {
                            Ok(())
                        }
                    })]
                >
                    <SliderLabel value=50.0>
                        "50"
                    </SliderLabel>
                </Slider>
            </Field>
            <Field label="Switch" name="switch">
                <Switch/>
            </Field>
            <div style="margin-top: 8px">
                <Button
                    button_type=ButtonType::Submit
                    on_click={
                        let field_context = FieldContextInjection::expect_context();
                        move |e: ev::MouseEvent| {
                            if !field_context.validate() {
                                e.prevent_default();
                            }
                        }
                    }
                >
                    "Submit"
                </Button>
            </div>
        </FieldContextProvider>
    </form>
}
```

### Required

```rust demo
view! {
    <form>
        <FieldContextProvider>
            <Field label="Username" name="username" required=true>
                <Input rules=vec![InputRule::required(true.into())]/>
            </Field>
            <div style="margin-top: 8px">
                <Button
                    button_type=ButtonType::Submit
                    on_click={
                        let field_context = FieldContextInjection::expect_context();
                        move |e: ev::MouseEvent| {
                            if !field_context.validate() {
                                e.prevent_default();
                            }
                        }
                    }
                >
                    "Submit"
                </Button>
            </div>
        </FieldContextProvider>
    </form>
}
```

### Field Props

| Name | Type | Default | Desciption |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| label | `MaybeProp<String>` | `Default::default()` | The label associated with the field. |
| name | `MaybeProp<String>` | `Default::default()` | A string specifying a name for the input control. This name is submitted along with the control's value when the form data is submitted. |
| orientation | `MaybeSignal<FieldOrientation>` | `FieldOrientation::Vertical` | The orientation of the label relative to the field component. |
| required | `MaybeSignal<bool>` | `false` | If set to true this field will be marked as required. |
| children | `Children` |  |  |

### FieldContextProvider Props

| Name     | Type       | Default | Desciption |
| -------- | ---------- | ------- | ---------- |
| children | `Children` |         |            |
