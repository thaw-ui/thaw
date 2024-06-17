use super::MenuInjection;
use crate::{theme::use_theme, Icon, Theme};
use leptos::*;
use thaw_components::{CSSTransition, OptionComp};
use thaw_utils::{class_list, mount_style, OptionalMaybeSignal, OptionalProp, StoredMaybeSignal};

#[component]
pub fn MenuItem(
    #[prop(into)] key: MaybeSignal<String>,
    #[prop(optional, into)] icon: OptionalMaybeSignal<icondata_core::Icon>,
    #[prop(into)] label: MaybeSignal<String>,
    #[prop(optional, into)] class: OptionalProp<MaybeSignal<String>>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    mount_style("menu-item", include_str!("./menu-item.css"));
    let theme = use_theme(Theme::light);

    let submenu_ref = NodeRef::<html::Div>::new();
    let is_children = children.is_some();
    let menu = MenuInjection::use_();
    let parent_menu_item = StoredValue::new(MenuItemInjection::use_());

    let is_open_children = RwSignal::new({
        key.with_untracked(|key| {
            menu.default_expanded_keys
                .with_value(|default_expanded_keys| default_expanded_keys.contains(key))
        })
    });
    let key: StoredMaybeSignal<_> = key.into();
    let is_selected = Memo::new(move |_| menu.value.with(|value| key.with(|key| value == key)));
    let is_submenu_selected =
        Memo::new(move |_| menu.path.with(|path| key.with(|key| path.contains(key))));

    let on_click = move |_| {
        if is_children {
            is_open_children.set(!is_open_children.get_untracked());
        } else {
            if !is_selected.get_untracked() {
                menu.path.update(|path| {
                    path.clear();
                });
                parent_menu_item.with_value(|parent_menu_item| {
                    if let Some(parent_menu_item) = parent_menu_item {
                        let mut item_path = vec![];
                        parent_menu_item.get_path(&mut item_path);
                        menu.path.update(|path| {
                            path.extend(item_path);
                        });
                    }
                });

                menu.value.set(key.get_untracked());
            }
        }
    };

    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let font_color = theme.common.color_primary.clone();
            css_vars.push_str(&format!("--thaw-font-color-active: {font_color};"));
            css_vars.push_str(&format!("--thaw-font-color: {};", theme.menu.color));
            css_vars.push_str(&format!("--thaw-background-color: {font_color}1a;"));
            css_vars.push_str(&format!(
                "--thaw-background-color-hover: {};",
                theme.menu.item_color_hover
            ));
        });
        css_vars
    });

    view! {
        <div class="thaw-menu-item">
            <div
                class=class_list![
                    "thaw-menu-item__content",
                    ("thaw-menu-item__content--selected", move || is_selected.get()),
                    ("thaw-menu-item__content--submenu-selected", move || is_submenu_selected.get()),
                    class.map(| c | move || c.get())
                ]

                on:click=on_click
                style=move || css_vars.get()
            >
                {
                    move || {
                        view! {
                            <OptionComp value=icon.get() let:icon>
                                <Icon icon=icon style="font-size: 18px;margin-right: 8px"/>
                            </OptionComp>
                        }
                    }
                }
                {move || label.get()}
                {
                    if children.is_some() {
                        view! {
                            <Icon
                                icon=icondata_ai::AiRightOutlined
                                class=Signal::derive(move || {
                                    let mut class = String::from("thaw-menu-item__arrow");
                                    if is_open_children.get() {
                                        class.push_str(" thaw-menu-item__arrow--open");
                                    }
                                    class
                                })/>
                        }.into()
                    } else {
                        None
                    }
                }
            </div>

            <OptionComp value=children let:children>
                <Provider value=MenuItemInjection { key, parent_menu_item }>
                    <CSSTransition
                        node_ref=submenu_ref
                        name="fade-in-height-expand-transition"
                        appear=is_open_children.get_untracked()
                        show=is_open_children
                        let:display
                    >
                        <div
                            class="thaw-menu-submenu"
                            style=move || display.get()
                            ref=submenu_ref
                            role="menu"
                            aria-expanded=move || if is_open_children.get() { "true" } else { "false" }
                        >
                            {children()}
                        </div>
                    </CSSTransition>
                </Provider>
            </OptionComp>
        </div>
    }
}

#[derive(Clone)]
struct MenuItemInjection {
    pub key: StoredMaybeSignal<String>,
    pub parent_menu_item: StoredValue<Option<MenuItemInjection>>,
}

impl MenuItemInjection {
    fn use_() -> Option<Self> {
        use_context()
    }

    fn get_path(&self, path: &mut Vec<String>) {
        self.parent_menu_item.with_value(|parent_menu_item| {
            if let Some(parent_menu_item) = parent_menu_item.as_ref() {
                parent_menu_item.get_path(path);
            }
        });

        path.push(self.key.get_untracked());
    }
}
