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
            <Field name="radio">
                <RadioGroup rules=vec![RadioGroupRule::required(true.into())] >
                    <Radio label="0" value="false"/>
                    <Radio label="1" value="true"/>
                </RadioGroup>
            </Field>
            <Field name="combobox">
                <Combobox rules=vec![ComboboxRule::required(true.into())] placeholder="Select an animal" clearable=true>
                    <ComboboxOption value="cat" text="Cat"/>
                    <ComboboxOption value="dog" text="Dog" />
                </Combobox>
            </Field>
            <Field name="spinbutton">
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
            <Field name="date">
                <DatePicker rules=vec![DatePickerRule::required(true.into())]/>
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
