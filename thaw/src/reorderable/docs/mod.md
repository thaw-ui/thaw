# Reorderable

```rust demo
use leptos::ev;

#[derive(Clone, Debug, PartialEq, Eq)]
struct Panel {
    id: u32,
    title: String,
}

let panels = RwSignal::new(vec![
    Panel {
        id: 1,
        title: "Panel #1".to_string(),
    },
    Panel {
        id: 2,
        title: "Panel #2".to_string(),
    },
    Panel {
        id: 3,
        title: "Panel #3".to_string(),
    },
]);
let col_panel_order = [
    // Column 1
    RwSignal::new(vec!["1".into(), "3".into()]),
    // Column 2
    RwSignal::new(vec!["2".into()]),
];

let row_panel_order = [
    // Column 1
    RwSignal::new(vec!["3".into(), "2".into()]),
    // Column 2
    RwSignal::new(vec!["1".into()]),
];

let add_panel = {
    move |_: ev::MouseEvent| {
        let mut panels = panels.write();
        let next_id = panels.last().map(|item| item.id).unwrap_or(0) + 1;
        let panel = Panel {
            id: next_id,
            title: format!("Panel #{}", next_id),
        };
        panels.push(panel);
        col_panel_order[0].update(|order| {
            order.insert(0, next_id.to_string().into());
        });
        row_panel_order[0].update(|order| {
            order.insert(0, next_id.to_string().into());
        });
    }
};

let col_panel_fn = move |id: Oco<'static, str>| {
    ViewFn::from(move || panels
        .read()
        .iter()
        .find(|p| p.id.to_string() == id)
        .map(|p| {let title = p.title.clone(); view!{<Tag>{title}</Tag>}})
        .expect("Not to receive an invalid id")
    )
};

let row_panel_fn = col_panel_fn.clone();

let col_order = Signal::derive(move || {
    col_panel_order
        .iter()
        .map(|id_list| {
            let id_string = id_list
                .read()
                .iter()
                .map(|id: &Oco<'static, str>| id.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            let val = format!(
                "{}",
                if !id_string.is_empty() {
                    id_string
                } else {
                    "Empty".to_string()
                }
            );
            view! { <li>{val}</li> }
        })
        .collect_view()
});
let row_order = Signal::derive(move || {
    row_panel_order
        .iter()
        .map(|id_list| {
            let id_string = id_list
                .read()
                .iter()
                .map(|id: &Oco<'static, str>| id.as_str())
                .collect::<Vec<&str>>()
                .join(", ");
            let val = format!(
                "{}",
                if !id_string.is_empty() {
                    id_string
                } else {
                    "Empty".to_string()
                }
            );
            view! { <li>{val}</li> }
        })
        .collect_view()
});
view! {
    <button on:click=add_panel>"Add a Panel to both reorderable containers"</button>
    <h2 class="background-info">"Column-directed reorderable"</h2>
    <Reorderable panel_order=col_panel_order panel_fn=col_panel_fn panel_class="panel"/>
    <h3 class="background-info">"Column-directed sort-order"</h3>
    <ol class="background-info">{col_order}</ol>
    <hr/>
    <h2 class="background-info">"Row-directed reorderable"</h2>
    <Reorderable
        panel_order=row_panel_order
        panel_fn=row_panel_fn
        horizontal=true
        panel_class="panel"
    />
    <h3 class="background-info">"Row-directed sort-order"</h3>
    <ol class="background-info">{row_order}</ol>
}
```


### Reorderable Props

| Name | Type | Default | Description |
| --- | --- | --- | --- |
| panel_order | `[RwSignal<Vec<Oco<'static, str>; COLUMNS]` |  | Configure how many rows or columns of containers the panels may be re-ordered into. |
| panel_fn | `Fn(Oco<'static, str>) -> ViewFn + Send + 'static + Clone` |  | Accessor for the panel contents given a panel ID. |
| horizontal | `bool` | `false` | whether the reorderable containers will be column oriented [default] or row-oriented. |
| container_class | `MaybeProp<String>` | `Default::default()` |  |
| panel_class | `MaybeProp<String>` | `Default::default()` |  |

