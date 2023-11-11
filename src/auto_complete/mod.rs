mod theme;

use crate::{
    components::{Binder, Follower, FollowerPlacement},
    mount_style, use_theme,
    utils::StoredMaybeSignal,
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

    let is_show_menu = create_rw_signal(false);
    let auto_complete_ref = create_node_ref::<html::Div>();
    let options = StoredMaybeSignal::from(options);
    let allow_value = move |_| {
        if !is_show_menu.get_untracked() {
            is_show_menu.set(true);
        }
        true
    };

    view! {
        <Binder target_ref=auto_complete_ref>
            <div class="thaw-auto-complete" ref=auto_complete_ref>
                <Input
                    value
                    placeholder
                    on_focus=move |_| is_show_menu.set(true)
                    on_blur=move |_| is_show_menu.set(false)
                    allow_value
                />
            </div>
            <Follower slot show=is_show_menu placement=FollowerPlacement::BottomStart>
                <div
                    class="thaw-auto-complete__menu"
                    style=move || menu_css_vars.get()
                >

                    {move || {
                        options
                            .get()
                            .into_iter()
                            .map(|v| {
                                let AutoCompleteOption { value: option_value, label } = v;
                                let on_click = move |_| {
                                    if clear_after_select.get_untracked() {
                                        value.set(String::new());
                                    } else {
                                        value.set(option_value.clone());
                                    }
                                    if let Some(on_select) = on_select {
                                        on_select.call(option_value.clone());
                                    }
                                    is_show_menu.set(false);
                                };
                                let on_mousedown = move |ev: ev::MouseEvent| {
                                    ev.prevent_default();
                                };
                                view! {
                                    <div
                                        class="thaw-auto-complete__menu-item"
                                        on:click=on_click
                                        on:mousedown=on_mousedown
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
