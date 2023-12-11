mod theme;

use crate::{
    components::{Binder, Follower, FollowerPlacement, FollowerWidth},
    use_theme,
    utils::{mount_style, StoredMaybeSignal},
    Input, Theme,
};
use leptos::*;
pub use theme::AutoCompleteTheme;

#[derive(Clone, PartialEq)]
pub struct AutoCompleteOption {
    pub label: String,
    pub value: String,
}

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] value: RwSignal<String>,
    #[prop(optional, into)] placeholder: MaybeSignal<String>,
    #[prop(optional, into)] options: MaybeSignal<Vec<AutoCompleteOption>>,
    #[prop(optional, into)] clear_after_select: MaybeSignal<bool>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: MaybeSignal<String>,
) -> impl IntoView {
    mount_style("auto-complete", include_str!("./auto-complete.css"));
    let theme = use_theme(Theme::light);
    let menu_css_vars = create_memo(move |_| {
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
        });
        css_vars
    });

    let select_option_index = create_rw_signal::<usize>(0);
    let menu_ref = create_node_ref::<html::Div>();
    let is_show_menu = create_rw_signal(false);
    let auto_complete_ref = create_node_ref::<html::Div>();
    let options = StoredMaybeSignal::from(options);
    let open_menu = move || {
        select_option_index.set(0);
        is_show_menu.set(true);
    };
    let allow_value = move |_| {
        if !is_show_menu.get_untracked() {
            open_menu();
        }
        true
    };

    let select_value = move |option_value: String| {
        if clear_after_select.get_untracked() {
            value.set(String::new());
        } else {
            value.set(option_value.clone());
        }
        if let Some(on_select) = on_select {
            on_select.call(option_value);
        }
        is_show_menu.set(false);
    };

    let on_keydown = move |event: ev::KeyboardEvent| {
        if !is_show_menu.get_untracked() {
            return;
        }
        let key = event.key();
        if key == "ArrowDown".to_string() {
            select_option_index.update(|index| {
                if *index == options.with_untracked(|options| options.len()) - 1 {
                    *index = 0
                } else {
                    *index += 1
                }
            });
        } else if key == "ArrowUp".to_string() {
            select_option_index.update(|index| {
                if *index == 0 {
                    *index = options.with_untracked(|options| options.len()) - 1;
                } else {
                    *index -= 1
                }
            });
        } else if key == "Enter".to_string() {
            let option_value = options.with_untracked(|options| {
                let index = select_option_index.get_untracked();
                if options.len() > index {
                    let option = &options[index];
                    Some(option.value.clone())
                } else {
                    None
                }
            });
            if let Some(option_value) = option_value {
                select_value(option_value);
            }
        }
    };

    view! {
        <Binder target_ref=auto_complete_ref>
            <div class=move || class.get() class:thaw-auto-complete=true ref=auto_complete_ref on:keydown=on_keydown>
                <Input
                    value
                    placeholder
                    disabled
                    invalid
                    on_focus=move |_| open_menu()
                    on_blur=move |_| is_show_menu.set(false)
                    allow_value
                />
            </div>
            <Follower
                slot
                show=is_show_menu
                placement=FollowerPlacement::BottomStart
                width=FollowerWidth::Target
            >
                <div
                    class="thaw-auto-complete__menu"
                    style=move || menu_css_vars.get()
                    ref=menu_ref
                >

                    {move || {
                        options
                            .get()
                            .into_iter()
                            .enumerate()
                            .map(|(index, v)| {
                                let AutoCompleteOption { value: option_value, label } = v;
                                let menu_item_ref = create_node_ref::<html::Div>();
                                let on_click = move |_| {
                                    select_value(option_value.clone());
                                };
                                let on_mouseenter = move |_| {
                                    select_option_index.set(index);
                                };
                                let on_mousedown = move |ev: ev::MouseEvent| {
                                    ev.prevent_default();
                                };
                                create_effect(move |_| {
                                    if index == select_option_index.get() {
                                        if !is_show_menu.get() {
                                            return;
                                        }
                                        if let Some(menu_item_ref) = menu_item_ref.get() {
                                            let menu_ref = menu_ref.get().unwrap();
                                            let menu_rect = menu_ref.get_bounding_client_rect();
                                            let item_rect = menu_item_ref.get_bounding_client_rect();
                                            if item_rect.y() < menu_rect.y() {
                                                menu_item_ref.scroll_into_view_with_bool(true);
                                            } else if item_rect.y() + item_rect.height()
                                                > menu_rect.y() + menu_rect.height()
                                            {
                                                menu_item_ref.scroll_into_view_with_bool(false);
                                            }
                                        }
                                    }
                                });
                                view! {
                                    <div
                                        class="thaw-auto-complete__menu-item"
                                        class=(
                                            "thaw-auto-complete__menu-item--selected",
                                            move || index == select_option_index.get(),
                                        )

                                        on:click=on_click
                                        on:mousedown=on_mousedown
                                        on:mouseenter=on_mouseenter
                                        ref=menu_item_ref
                                    >
                                        {label}
                                    </div>
                                }
                            })
                            .collect_view()
                    }}

                </div>
            </Follower>
        </Binder>
    }
}


