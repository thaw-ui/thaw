use crate::ConfigInjection;
use leptos::*;
use thaw_components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth};
use thaw_utils::{mount_style, Model};

#[component]
pub fn Combobox(
    #[prop(optional, into)] value: Model<Vec<String>>,
    #[prop(optional)] multiselect: bool,
    children: Children,
) -> impl IntoView {
    mount_style("combobox", include_str!("./combobox.css"));
    let config_provider = ConfigInjection::use_();
    let trigger_ref = NodeRef::<html::Div>::new();
    let listbox_ref = NodeRef::<html::Div>::new();
    let is_show_listbox = RwSignal::new(false);

    view! {
        <Binder target_ref=trigger_ref>
            <div
                class="thaw-combobox"
                ref=trigger_ref
                on:click=move |_| {
                    is_show_listbox.set(true);
                }
            >
                <input
                    type="text"
                    aria-expanded="true"
                    role="combobox"
                    class="thaw-combobox__input"
                    prop:value=move || {
                        value.with(|value| {
                            if multiselect {
                                None
                            } else {
                                value.first().cloned()
                            }
                        })
                    }
                />
                <span
                    aria-expanded="true"
                    role="button"
                    aria-label="Open"
                    class="thaw-combobox__expand-icon"
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
                <Provider value=ComboboxInjection{value, multiselect}>
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
                            ref=listbox_ref
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
    value: Model<Vec<String>>,
    pub multiselect: bool,
}

impl ComboboxInjection {
    pub fn use_() -> Self {
        expect_context()
    }

    pub fn is_selected(&self, key: &String) -> bool {
        self.value.with(|value| value.contains(key))
    }

    pub fn on_option_select(&self, key: &String) {
        self.value.update(|value| {
            if self.multiselect {
                if let Some(index) = value.iter().position(|k| k == key) {
                    value.remove(index);
                    return;
                }
            } else {
                value.clear();
            }
            value.push(key.clone());
        });
    }
}
