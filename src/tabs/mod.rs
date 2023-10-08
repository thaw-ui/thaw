mod tab;
use crate::{theme::use_theme, utils::mount_style::mount_style, Theme};
use leptos::*;

pub use tab::*;

#[component]
pub fn Tabs(active_key: RwSignal<&'static str>, children: Children) -> impl IntoView {
    mount_style("tabs", include_str!("./tabs.css"));
    let tab_options_vec = create_rw_signal(vec![]);
    provide_context(TabsInjectionKey {
        active_key: active_key.clone(),
        tab_options_vec: tab_options_vec.clone(),
    });
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        let theme = theme.get();
        let color_primary = theme.common.color_primary.clone();
        css_vars.push_str(&format!(
            "--label-active-background-color: {color_primary};"
        ));
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

    view! {
        <div class="melt-tabs" style=move || css_vars.get()>
            <div class="melt-tabs__label-list" ref=label_list_ref>
                <For
                    each=move || tab_options_vec.get()
                    key=move |v| v.key
                    children=move |options| {
                        let label_ref = create_node_ref::<html::Span>();
                        create_effect(move |_| {
                            let Some(label) = label_ref.get() else {
                                return;
                            };
                            let Some(label_list) = label_list_ref.get() else {
                                return;
                            };
                            if options.key == active_key.get() {
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
                        });
                        view! {
                            <span
                                class="melt-tabs__label"
                                class=(
                                    "melt-tabs__label--active",
                                    move || options.key == active_key.get(),
                                )

                                on:click=move |_| active_key.set(options.key)
                                ref=label_ref
                            >
                                {options.label}
                            </span>
                        }
                    }
                />

                <span class="melt-tabs-label__line" style=move || label_line_style.get()></span>
            </div>
            <div>{children()}</div>
        </div>
    }
}

#[derive(Clone)]
pub(crate) struct TabsLabelLine {
    width: f64,
    left: f64,
}

#[derive(Clone)]
pub struct TabsInjectionKey {
    active_key: RwSignal<&'static str>,
    tab_options_vec: RwSignal<Vec<TabOptions>>,
}

impl TabsInjectionKey {
    pub fn get_key(&self) -> &str {
        self.active_key.get()
    }

    pub(crate) fn push_tab_options(&self, options: TabOptions) {
        self.tab_options_vec.update(|v| {
            v.push(options);
        });
    }
}

pub fn use_tabs() -> TabsInjectionKey {
    use_context::<TabsInjectionKey>().expect("TabsInjectionKey not exist")
}
