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

### validate

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
            <button
                type="submit"
                on:click={
                    let field_context = FieldContextInjection::expect_context();
                    move |e| {
                        if !field_context.validate() {
                            e.prevent_default();
                        }
                    }
                }
            >
                "Submit"
            </button>
        </FieldContextProvider>
    </form>
}
```
