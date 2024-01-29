mod tab;

use crate::{
    theme::use_theme,
    utils::{class_list::class_list, mount_style, Model},
    Theme,
};
use leptos::*;

pub use tab::*;

#[component]
pub fn Tabs(
    #[prop(optional, into)] value: Model<String>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tabs", include_str!("./tabs.css"));
    let tab_options_vec = create_rw_signal(vec![]);

    view! {
        <Provider value=TabsInjection {
            active_key: value,
            tab_options_vec,
        }>
            <TabsInner class value tab_options_vec children/>
        </Provider>
    }
}

#[component]
fn TabsInner(
    value: Model<String>,
    tab_options_vec: RwSignal<Vec<TabOption>>,
    #[prop(optional, into)] class: MaybeSignal<String>,
    children: Children,
) -> impl IntoView {
    mount_style("tabs", include_str!("./tabs.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            let color_primary = theme.common.color_primary.clone();
            css_vars.push_str(&format!(
                "--thaw-label-active-background-color: {color_primary};"
            ));
        });
        css_vars
    });

    let label_line = create_rw_signal::<Option<TabsLabelLine>>(None);
    let label_line_style = create_memo(move |_| {
        let mut style = String::new();
        if let Some(line) = label_line.get() {
            style.push_str(&format!("width: {}px; left: {}px", line.width, line.left))
        }
        style
    });
    let label_list_ref = create_node_ref::<html::Div>();

    let children = children();

    view! {
        <div class=class_list!["thaw-tabs", move || class.get()] style=move || css_vars.get()>
            <div class="thaw-tabs__label-list" ref=label_list_ref role="tablist">
                <For
                    each=move || tab_options_vec.get()
                    key=move |v| v.key.clone()
                    children=move |option| {
                        let label_ref = create_node_ref::<html::Span>();
                        let TabOption { key, label, label_view } = option;
                        create_effect({
                            let key = key.clone();
                            move |_| {
                                let Some(label) = label_ref.get() else {
                                    return;
                                };
                                let Some(label_list) = label_list_ref.get() else {
                                    return;
                                };
                                if key.clone() == value.get() {
                                    request_animation_frame(move || {
                                        let list_rect = label_list.get_bounding_client_rect();
                                        let rect = label.get_bounding_client_rect();
                                        label_line
                                            .set(
                                                Some(TabsLabelLine {
                                                    width: rect.width(),
                                                    left: rect.left() - list_rect.left(),
                                                }),
                                            );
                                    });
                                }
                            }
                        });
                        let is_active = create_memo({
                            let key = key.clone();
                            move |_| key == value.get()
                        });
                        if let Some(label_view) = label_view {
                            let TabLabelView { class, children } = label_view;
                            view! {
                                <span
                                    class=class_list![
                                        "thaw-tabs__label", ("thaw-tabs__label--active", move ||
                                        is_active.get()), move || class.get()
                                    ]

                                    on:click={
                                        let key = key.clone();
                                        move |_| value.set(key.clone())
                                    }

                                    ref=label_ref
                                    role="tab"
                                    aria-selected=move || {
                                        if is_active.get() { "true" } else { "false" }
                                    }
                                >

                                    {children}
                                </span>
                            }
                        } else {
                            view! {
                                <span
                                    class="thaw-tabs__label"
                                    class=("thaw-tabs__label--active", move || is_active.get())

                                    on:click={
                                        let key = key.clone();
                                        move |_| value.set(key.clone())
                                    }

                                    ref=label_ref
                                    role="tab"
                                    aria-selected=move || {
                                        if is_active.get() { "true" } else { "false" }
                                    }
                                >

                                    {if label.is_empty() { key } else { label }}
                                </span>
                            }
                        }
                    }
                />

                <span class="thaw-tabs-label__line" style=move || label_line_style.get()></span>
            </div>
            <div>{children}</div>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct TabsLabelLine {
    width: f64,
    left: f64,
}

#[derive(Clone)]
pub(crate) struct TabsInjection {
    active_key: Model<String>,
    tab_options_vec: RwSignal<Vec<TabOption>>,
}

impl TabsInjection {
    pub fn get_key(&self) -> String {
        self.active_key.get()
    }

    pub(crate) fn push_tab_options(&self, options: TabOption) {
        self.tab_options_vec.update(|v| {
            v.push(options);
        });
    }

    pub(crate) fn remove_tab_options(&self, key: &String) {
        self.tab_options_vec.update(|v| {
            if let Some(index) = v.iter().position(|tab| &tab.key == key) {
                v.remove(index);
            }
        });
    }
}

pub(crate) fn use_tabs() -> TabsInjection {
    expect_context()
}
