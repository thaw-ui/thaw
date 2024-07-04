use crate::ConfigInjection;
use leptos::{context::Provider, ev, html, prelude::*};
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{add_event_listener, mount_style, Model};

#[component]
pub fn Combobox(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] selected_options: Model<Vec<String>>,
    #[prop(optional)] multiselect: bool,
    #[prop(optional)] clearable: bool,
    children: Children,
) -> impl IntoView {
    mount_style("combobox", include_str!("./combobox.css"));
    let config_provider = ConfigInjection::use_();
    let trigger_ref = NodeRef::<html::Div>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let is_show_listbox = RwSignal::new(false);

    let clear_icon_ref = NodeRef::<html::Span>::new();
    let is_show_clear_icon = Memo::new(move |_| {
        if clearable {
            selected_options.with(|options| !options.is_empty())
        } else {
            false
        }
    });
    if clearable {
        Effect::new(move |_| {
            let Some(clear_icon_el) = clear_icon_ref.get() else {
                return;
            };
            let handler = add_event_listener(clear_icon_el.into(), ev::click, move |e| {
                e.stop_propagation();
                selected_options.set(vec![]);
            });
            on_cleanup(move || handler.remove());
        });
    }

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
        let handle = window_event_listener(ev::click, move |ev| {
            use leptos::wasm_bindgen::__rt::IntoJsResult;
            if !is_show_listbox.get_untracked() {
                return;
            }
            let el = ev.target();
            let mut el: Option<web_sys::Element> =
                el.into_js_result().map_or(None, |el| Some(el.into()));
            let body = document().body().unwrap();
            if let Some(listbox_el) = listbox_ref.get_untracked() {
                if let Some(trigger_el) = trigger_ref.get_untracked() {
                    while let Some(current_el) = el {
                        if current_el == *body {
                            break;
                        };
                        if current_el == listbox_el {
                            return;
                        }
                        if current_el == trigger_el {
                            return;
                        }
                        el = current_el.parent_element();
                    }
                }
            }
            is_show_listbox.set(false);
        });
        on_cleanup(move || handle.remove());
    }

    let on_input = move |ev| {
        let input_value = event_target_value(&ev);
        if !multiselect {
            if selected_options.with_untracked(|options| {
                if let Some(option) = options.first() {
                    if option != &input_value {
                        return true;
                    }
                }
                false
            }) {
                selected_options.set(vec![]);
            }
        }
        value.set(input_value);
    };
    let on_blur = move |_| {
        if multiselect {
            value.set(String::new());
        } else {
            if selected_options.with_untracked(|options| options.is_empty()) {
                value.set(String::new());
            }
        }
    };

    view! {
        <Binder target_ref=trigger_ref>
            <div
                class="thaw-combobox"
                node_ref=trigger_ref
                on:click=move |_| {
                    is_show_listbox.update(|show| *show = !*show);
                }
            >
                <input
                    type="text"
                    aria-expanded="true"
                    role="combobox"
                    class="thaw-combobox__input"
                    prop:value=move || {
                        value.get()
                    }
                    on:input=on_input
                    on:blur=on_blur
                />
                {
                    if clearable {
                        view! {
                            <span
                                aria-hidden="true"
                                class="thaw-combobox__clear-icon"
                                style=move || (!is_show_clear_icon.get()).then(|| "display: none")
                                node_ref=clear_icon_ref
                            >
                                <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                                    <path d="m4.09 4.22.06-.07a.5.5 0 0 1 .63-.06l.07.06L10 9.29l5.15-5.14a.5.5 0 0 1 .63-.06l.07.06c.18.17.2.44.06.63l-.06.07L10.71 10l5.14 5.15c.18.17.2.44.06.63l-.06.07a.5.5 0 0 1-.63.06l-.07-.06L10 10.71l-5.15 5.14a.5.5 0 0 1-.63.06l-.07-.06a.5.5 0 0 1-.06-.63l.06-.07L9.29 10 4.15 4.85a.5.5 0 0 1-.06-.63l.06-.07-.06.07Z" fill="currentColor"></path>
                                </svg>
                            </span>
                        }.into()
                    } else {
                        None
                    }
                }
                <span
                    aria-expanded="true"
                    role="button"
                    aria-label="Open"
                    class="thaw-combobox__expand-icon"
                    style=move || is_show_clear_icon.get().then(|| "display: none")
                >
                    <svg fill="currentColor" aria-hidden="true" width="1em" height="1em" viewBox="0 0 20 20">
                        <path d="M15.85 7.65c.2.2.2.5 0 .7l-5.46 5.49a.55.55 0 0 1-.78 0L4.15 8.35a.5.5 0 1 1 .7-.7L10 12.8l5.15-5.16c.2-.2.5-.2.7 0Z" fill="currentColor">
                        </path>
                    </svg>
                </span>
            </div>
            <Follower
                slot
                show=is_show_listbox
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::MinTarget
            >
                <Provider value=ComboboxInjection{value, multiselect, selected_options, is_show_listbox}>
                    <CSSTransition
                        node_ref=listbox_ref
                        name="fade-in-scale-up-transition"
                        appear=is_show_listbox.get_untracked()
                        show=is_show_listbox
                        let:display
                    >
                        <div
                            class="thaw-config-provider thaw-combobox__listbox"
                            style=move || display.get()
                            data-thaw-id=config_provider.id().clone()
                            node_ref=listbox_ref
                            role="listbox"
                        >
                            {children()}
                        </div>
                    </CSSTransition>
                </Provider>
            </Follower>
        </Binder>
    }
}

#[derive(Clone, Copy)]
pub(crate) struct ComboboxInjection {
    value: Model<String>,
    selected_options: Model<Vec<String>>,
    is_show_listbox: RwSignal<bool>,
    pub multiselect: bool,
}

impl ComboboxInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn is_selected(&self, value: &String) -> bool {
        self.selected_options
            .with(|options| options.contains(value))
    }

    pub fn on_option_select(&self, value: &String, text: &String) {
        self.selected_options.update(|options| {
            if self.multiselect {
                if let Some(index) = options.iter().position(|v| v == value) {
                    options.remove(index);
                    return;
                }
                options.push(value.clone());
            } else {
                *options = vec![value.clone()];
                self.value.set(text.clone());
                self.is_show_listbox.set(false);
            }
        });
    }
}
