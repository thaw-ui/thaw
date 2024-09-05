# Tag Picker

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <TagPicker selected_options>
        <TagPickerControl slot>
            <TagPickerGroup>
                {move || {
                    selected_options.get().into_iter().map(|option| view!{
                        <Tag>
                            {option}
                        </Tag>
                    }).collect_view()
                }}
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