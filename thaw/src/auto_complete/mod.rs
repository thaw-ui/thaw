mod theme;

pub use theme::AutoCompleteTheme;

use crate::{
    components::{Binder, CSSTransition, Follower, FollowerPlacement, FollowerWidth},
    use_theme, ComponentRef, Input, InputPrefix, InputRef, InputSuffix, Theme,
};
use leptos::*;
use thaw_utils::{class_list, mount_style, Model, OptionalProp, StoredMaybeSignal};

#[derive(Clone, PartialEq)]
pub struct AutoCompleteOption {
    pub label: String,
    pub value: String,
}

#[slot]
pub struct AutoCompletePrefix {
    children: Children,
}

#[slot]
pub struct AutoCompleteSuffix {
    children: Children,
}

#[component]
pub fn AutoComplete(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] placeholder: OptionalProp<MaybeSignal<String>>,
    #[prop(optional, into)] options: MaybeSignal<Vec<AutoCompleteOption>>,
    #[prop(optional, into)] clear_after_select: MaybeSignal<bool>,
    #[prop(optional, into)] blur_after_select: MaybeSignal<bool>,
    #[prop(optional, into)] on_select: Option<Callback<String>>,
    #[prop(optional, into)] disabled: MaybeSignal<bool>,
    #[prop(optional, into)] allow_free_input: bool,
    #[prop(optional, into)] invalid: MaybeSignal<bool>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] auto_complete_prefix: Option<AutoCompletePrefix>,
    #[prop(optional)] auto_complete_suffix: Option<AutoCompleteSuffix>,
    #[prop(optional)] comp_ref: ComponentRef<AutoCompleteRef>,
    #[prop(attrs)] attrs: Vec<(&'static str, Attribute)>,
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

    let input_ref = ComponentRef::<InputRef>::new();

    let default_index = if allow_free_input { None } else { Some(0) };

    let select_option_index = create_rw_signal::<Option<usize>>(default_index);
    let menu_ref = create_node_ref::<html::Div>();
    let is_show_menu = create_rw_signal(false);
    let auto_complete_ref = create_node_ref::<html::Div>();
    let options = StoredMaybeSignal::from(options);
    let open_menu = move || {
        select_option_index.set(default_index);
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
        if allow_free_input {
            select_option_index.set(None);
        }
        is_show_menu.set(false);
        if blur_after_select.get_untracked() {
            if let Some(input_ref) = input_ref.get_untracked() {
                input_ref.blur();
            }
        }
    };

    // we unset selection index whenever options get changed
    // otherwise e.g. selection could move from one item to
    // another staying on the same index
    create_effect(move |_| {
        options.track();
        select_option_index.set(default_index);
    });

    let on_keydown = move |event: ev::KeyboardEvent| {
        if !is_show_menu.get_untracked() {
            return;
        }
        let key = event.key();
        if key == *"ArrowDown" {
            select_option_index.update(|index| {
                if *index == Some(options.with_untracked(|options| options.len()) - 1) {
                    *index = default_index
                } else {
                    *index = Some(index.map_or(0, |index| index + 1))
                }
            });
        } else if key == *"ArrowUp" {
            select_option_index.update(|index| {
                match *index {
                    None => *index = Some(options.with_untracked(|options| options.len()) - 1),
                    Some(0) => {
                        if allow_free_input {
                            *index = None
                        } else {
                            *index = Some(options.with_untracked(|options| options.len()) - 1)
                        }
                    }
                    Some(prev_index) => *index = Some(prev_index - 1),
                };
            });
        } else if key == *"Enter" {
            event.prevent_default();
            let option_value = options.with_untracked(|options| {
                let index = select_option_index.get_untracked();
                match index {
                    None if allow_free_input => {
                        let value = value.get_untracked();
                        (!value.is_empty()).then_some(value)
                    }
                    Some(index) if options.len() > index => {
                        let option = &options[index];
                        Some(option.value.clone())
                    }
                    _ => None,
                }
            });
            if let Some(option_value) = option_value {
                select_value(option_value);
            }
        }
    };
    input_ref.on_load(move |_| {
        comp_ref.load(AutoCompleteRef { input_ref });
    });

    view! {
        <Binder target_ref=auto_complete_ref>
            <div
                class=class_list!["thaw-auto-complete", class.map(| c | move || c.get())]
                ref=auto_complete_ref
                on:keydown=on_keydown
            >
                <Input
                    attrs
                    value
                    placeholder
                    disabled
                    invalid
                    on_focus=move |_| open_menu()
                    on_blur=move |_| is_show_menu.set(false)
                    allow_value
                    comp_ref=input_ref
                >
                    <InputPrefix if_=auto_complete_prefix.is_some() slot>

                        {if let Some(auto_complete_prefix) = auto_complete_prefix {
                            Some((auto_complete_prefix.children)())
                        } else {
                            None
                        }}

                    </InputPrefix>
                    <InputSuffix if_=auto_complete_suffix.is_some() slot>

                        {if let Some(auto_complete_suffix) = auto_complete_suffix {
                            Some((auto_complete_suffix.children)())
                        } else {
                            None
                        }}

                    </InputSuffix>
                </Input>
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
                    show=is_show_menu
                    let:display
                >
                    <div
                        class="thaw-auto-complete__menu"
                        style=move || display.get().map(|d| d.to_string()).unwrap_or_else(|| menu_css_vars.get())
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
                                        select_option_index.set(Some(index));
                                    };
                                    let on_mousedown = move |ev: ev::MouseEvent| {
                                        ev.prevent_default();
                                    };
                                    create_effect(move |_| {
                                        if Some(index) == select_option_index.get() {
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
                                                move || Some(index) == select_option_index.get(),
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
                </CSSTransition>
            </Follower>
        </Binder>
    }
}

#[derive(Clone)]
pub struct AutoCompleteRef {
    input_ref: ComponentRef<InputRef>,
}

impl AutoCompleteRef {
    pub fn focus(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.focus();
        }
    }

    pub fn blur(&self) {
        if let Some(input_ref) = self.input_ref.get_untracked() {
            input_ref.blur();
        }
    }
}
