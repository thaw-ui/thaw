use super::SelectOption;
use crate::{theme::use_theme, Theme};
use leptos::*;
use std::hash::Hash;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{class_list, mount_style, Model, OptionalProp};

#[component]
pub fn Multiselect<T>(
    #[prop(optional, into)] value: Model<Vec<T>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<SelectOption<T>>>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
) -> impl IntoView
where
    T: Eq + Hash + Clone + 'static,
{
    mount_style("multiselect", include_str!("./multiselect.css"));

    let theme = use_theme(Theme::light);
    let css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let border_color_hover = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-border-color-hover: {border_color_hover};"));
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.background_color
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-border-color: {};",
                theme.select.border_color
            ));
        });

        css_vars
    });

    let menu_css_vars = Memo::new(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.select.menu_background_color
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.select.menu_background_color_hover
            ));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.select.font_color));
            css_vars.push_str(&format!(
                "--thaw-font-color-selected: {};",
                theme.common.color_primary
            ));
        });
        css_vars
    });

    let is_show_menu = create_rw_signal(false);
    let trigger_ref = create_node_ref::<html::Div>();
    let menu_ref = create_node_ref::<html::Div>();
    let show_menu = move |_| {
        is_show_menu.set(true);
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        use leptos::wasm_bindgen::__rt::IntoJsResult;
        let timer = window_event_listener(ev::click, move |ev| {
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            while let Some(current_el) = el {
                if current_el == *body {
                    break;
                };
                if current_el == ***menu_ref.get().unwrap() {
                    return;
                }
                el = current_el.parent_element();
            }
            is_show_menu.set(false);
        });
        on_cleanup(move || timer.remove());
    }
    view! {
        <Binder target_ref=trigger_ref>
            <div
                class=class_list!["thaw-multiselect", class.map(| c | move || c.get())]
                ref=trigger_ref
                on:click=show_menu
                style=move || css_vars.get()
            >
            </div>
            <Follower
                slot
                show=is_show_menu
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <CSSTransition
                    node_ref=menu_ref
                    name="fade-in-scale-up-transition"
                    appear=is_show_menu.get_untracked()
                    show=is_show_menu
                    let:display
                >
                    <div
                        class="thaw-multiselect-menu"
                        style=move || {
                            display
                                .get()
                                .map(|d| d.to_string())
                                .unwrap_or_else(|| menu_css_vars.get())
                        }

                        ref=menu_ref
                    >
                        <For
                            each=move || options.get()
                            key=move |item| item.value.clone()
                            children=move |item| {
                                let SelectOption { value: item_value, label } = item;
                                let item_value = StoredValue::new(item_value);
                                let is_selected = Memo::new(move |_| item_value.with_value(|item_value| value.with(|value| value.iter().any(|item| item == item_value))));
                                view! {
                                    <div
                                        class="thaw-multiselect-menu__item"
                                        class=(
                                            "thaw-multiselect-menu__item-selected",
                                            move || is_selected.get()
                                        )

                                        on:click=move |_| {
                                            value.update(|value|
                                                if is_selected.get_untracked() {

                                                } else {
                                                    value.push(item_value.get_value());
                                                }
                                            )
                                        }
                                    >
                                        {label}
                                    </div>
                                }
                            }
                        />

                    </div>
                </CSSTransition>
            </Follower>
        </Binder>
    }
}
