use crate::teleport::Teleport;
use leptos::*;
use std::hash::Hash;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<T> {
    pub label: String,
    pub value: T,
}

#[component]
pub fn Select<T>(
    cx: Scope,
    #[prop(into)] value: RwSignal<Option<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    let is_show_popover = create_rw_signal(cx, false);
    let trigger_ref = create_node_ref::<html::Div>(cx);
    let popover_ref = create_node_ref::<html::Div>(cx);
    let show_popover = move |_| {
        let rect = trigger_ref.get().unwrap().get_bounding_client_rect();
        is_show_popover.set(true);
        if let Some(popover_ref) = popover_ref.get() {
            popover_ref
                .style("width", format!("{}px", rect.width()))
                .style(
                    "transform",
                    format!(
                        "translateX({}px) translateY({}px) translateX(-50%)",
                        rect.x() + rect.width() / 2.0,
                        rect.y() + rect.height()
                    ),
                );
        }
    };

    let temp_options = options.clone();
    let select_option_label = create_memo(cx, move |_| match value.get() {
        Some(value) => temp_options
            .get()
            .iter()
            .find(move |v| v.value == value)
            .map_or(String::new(), |v| v.label.clone()),
        None => String::new(),
    });
    view! { cx,
        <div class="melt-select" ref=trigger_ref on:click=show_popover>
            {
                move || select_option_label.get()
            }
        </div>
        <Teleport>
            <div class="melt-select-menu" style=move || if is_show_popover.get() { "display: none" } else { "" } ref=popover_ref>
                <For
                    each=move || options.get()
                    key=move |item| item.value.clone()
                    view=move |cx, item| {
                        view! { cx,
                            <div class="melt-select-menu__item">
                                { item.label }
                            </div>
                        }
                    }
                >
                </For>
            </div>
        </Teleport>
    }
}
