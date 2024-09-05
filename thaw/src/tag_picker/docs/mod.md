# Tag Picker

```rust demo
view! {
    <TagPicker>
        <TagPickerControl slot>
            <TagPickerGroup>
                "123"
            </TagPickerGroup>
            <TagPickerInput />
        </TagPickerControl>
        <TagPickerOption value="cat">
            "Cat"
        </TagPickerOption>
        <TagPickerOption value="dog">
            "Dog"
        </TagPickerOption>
    </TagPicker>
}
```