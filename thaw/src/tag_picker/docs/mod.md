# Tag Picker

```rust demo
let selected_options = RwSignal::new(vec![]);
let options = vec!["Cat", "Dog"];

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
        {
            move || {
                selected_options.with(|selected_options| {
                    options.iter().filter_map(|option| {
                        if selected_options.iter().any(|o| o == option) {
                            return None
                        } else {
                            Some(view! {
                                <TagPickerOption value=option.clone() text=option.clone() />
                            })
                        }
                    }).collect_view()
                })
            }
        }
    </TagPicker>
}
```
