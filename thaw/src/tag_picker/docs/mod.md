# Tag Picker

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <TagPicker selected_options>
        <TagPickerControl slot>
            <TagPickerGroup>
                {move || {
                    selected_options.get().into_iter().map(|option| view!{
                        <Tag value=option.clone()>
                            {option}
                        </Tag>
                    }).collect_view()
                }}
            </TagPickerGroup>
            <TagPickerInput />
        </TagPickerControl>
        <TagPickerOption value="cat" text="Cat" />
        <TagPickerOption value="dog" text="Dog" />
    </TagPicker>
}
```
