use crate::{teleport::Teleport, theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;
use std::hash::Hash;
use stylers::style_sheet_str;
use wasm_bindgen::__rt::IntoJsResult;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SelectOption<T> {
    pub label: String,
    pub value: T,
}

#[component]
pub fn Select<T>(
    #[prop(into)] value: RwSignal<Option<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    let class_name = mount_style("select", || style_sheet_str!("./src/select/select.css"));

    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let bg_color = theme.common.color_primary;
        css_vars.push_str(&format!("--font-color: {bg_color};"));
        css_vars.push_str(&format!("--border-color-hover: {bg_color};"));

        css_vars
    });

    let is_show_popover = create_rw_signal(false);
    let trigger_ref = create_node_ref::<html::Div>();
    let popover_ref = create_node_ref::<html::Div>();
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
    let timer = window_event_listener(ev::click, move |ev| {
        let el = ev.target();
        let mut el: Option<web_sys::Element> =
            el.into_js_result().map_or(None, |el| Some(el.into()));
        let body = document().body().unwrap();
        while let Some(current_el) = el {
            if current_el == *body {
                break;
            };
            if current_el == ***popover_ref.get().unwrap()
                || current_el == ***trigger_ref.get().unwrap()
            {
                return;
            }
            el = current_el.parent_element();
        }
        is_show_popover.set(false);
    });
    on_cleanup(move || timer.remove());

    let temp_options = options.clone();
    let select_option_label = create_memo(move |_| match value.get() {
        Some(value) => temp_options
            .get()
            .iter()
            .find(move |v| v.value == value)
            .map_or(String::new(), |v| v.label.clone()),
        None => String::new(),
    });
    view! {  class=class_name,
        <div class="melt-select" ref=trigger_ref on:click=show_popover style=move || css_vars.get()>
            {
                move || select_option_label.get()
            }
        </div>
        <Teleport>
            <div class="melt-select-menu" style=move || if is_show_popover.get() { css_vars.get() } else { format!("display: none; {}", css_vars.get()) } ref=popover_ref>
                <For
                    each=move || options.get()
                    key=move |item| item.value.clone()
                    children=move | item| {
                        let item = store_value( item);
                        let onclick = move |_| {
                            let SelectOption { value: item_value, label: _ } = item.get_value();
                            value.set(Some(item_value));
                            is_show_popover.set(false);
                        };
                        view! {  class=class_name,
                            <div class="melt-select-menu__item" class=("melt-select-menu__item-selected", move || value.get() == Some(item.get_value().value) ) on:click=onclick>
                                { item.get_value().label }
                            </div>
                        }
                    }
                >
                </For>
            </div>
        </Teleport>
    }
}
