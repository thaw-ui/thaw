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

### Size

```rust demo
let selected_options = RwSignal::new(vec![]);

view! {
    <Flex vertical=true inline=true>
        <TagPicker selected_options size=TagPickerSize::ExtraLarge>
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
                let options = vec!["Cat", "Dog"];
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
        <TagPicker selected_options size=TagPickerSize::Large>
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
                let options = vec!["Cat", "Dog"];
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
                let options = vec!["Cat", "Dog"];
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
    </Flex>
}
```

### Grouped

```rust demo
use leptos::either::Either;
let selected_options = RwSignal::new(vec![]);
let land = vec!["Cat", "Dog", "Ferret", "Hamster"];
let water = vec!["Fish", "Jellyfish", "Octopus", "Seal"];

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
        {move || {
            selected_options.with(|selected_options| {
                let land_view = land.iter().filter_map(|option| {
                    if selected_options.iter().any(|o| o == option) {
                        return None
                    } else {
                        Some(view! {
                            <TagPickerOption value=option.clone() text=option.clone() />
                        })
                    }
                }).collect_view();
                if land_view.is_empty() {
                    Either::Left(())
                } else {
                    Either::Right(view! {
                        <TagPickerOptionGroup label="Land">
                            {land_view}
                        </TagPickerOptionGroup>
                    })
                }
            })
        }}
        {move || {
            selected_options.with(|selected_options| {
                let water_view = water.iter().filter_map(|option| {
                    if selected_options.iter().any(|o| o == option) {
                        return None
                    } else {
                        Some(view! {
                            <TagPickerOption value=option.clone() text=option.clone() />
                        })
                    }
                }).collect_view();
                if water_view.is_empty() {
                    Either::Left(())
                } else {
                    Either::Right(view! {
                        <TagPickerOptionGroup label="Sea">
                            {water_view}
                        </TagPickerOptionGroup>
                    })
                }
            })
        }}
    </TagPicker>
}
```

### TagPicker Props

| Name               | Type                    | Default                 | Description                       |
| ------------------ | ----------------------- | ----------------------- | --------------------------------- |
| class              | `MaybeProp<String>`     | `Default::default()`    |                                   |
| selected_option    | `Model<Vec<String>>`    | `Default::default()`    | An array of selected option keys. |
| size               | `Signal<TagPickerSize>` | `TagPickerSize::Medium` | The size of the TagPicker.        |
| tag_picker_control | slot `TagPickerControl` |                         |                                   |
| children           | `Children`              |                         |                                   |

### TagPickerGroup Props

| Name     | Type                | Default              | Description |
| -------- | ------------------- | -------------------- | ----------- |
| class    | `MaybeProp<String>` | `Default::default()` |             |
| children | `Children`          |                      |             |

### TagPickerInput Props

| Name  | Type                | Default              | Description |
| ----- | ------------------- | -------------------- | ----------- |
| class | `MaybeProp<String>` | `Default::default()` |             |

### TagPickerOption Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| class | `MaybeProp<String>` | `Default::default()` |  |
| disable | `MaybeProp<bool>` | `false` | Sets an option to the disabled state. |
| value | `String` |  | Defines a unique identifier for the option. |
| text | `String` |  | An optional override the string value of the Option's display text, defaulting to the Option's child content. |
| children | `Option<Children>` | `None` |  |

### TagPickerOptionGroup Props

| Name     | Type                | Default              | Desciption          |
| -------- | ------------------- | -------------------- | ------------------- |
| class    | `MaybeProp<String>` | `Default::default()` |                     |
| label    | `String`            |                      | Label of the group. |
| children | `Children`          |                      |                     |
