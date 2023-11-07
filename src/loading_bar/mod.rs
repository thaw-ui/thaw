mod loading_bar_provider;

use crate::{mount_style, use_theme, utils::ComponentRef, Theme};
use leptos::*;
pub use loading_bar_provider::{use_loading_bar, LoadingBarProvider};

#[derive(Clone)]
pub(crate) struct LoadingBarRef {
    start: Callback<()>,
    finish: Callback<()>,
    error: Callback<()>,
}

impl LoadingBarRef {
    pub fn start(&self) {
        self.start.call(());
    }
    pub fn finish(&self) {
        self.finish.call(());
    }
    pub fn error(&self) {
        self.error.call(());
    }
}

#[component]
pub(crate) fn LoadingBar(#[prop(optional)] comp_ref: ComponentRef<LoadingBarRef>) -> impl IntoView {
    mount_style("loading-bar", include_str!("./loading-bar.css"));
    let theme = use_theme(Theme::light);
    let css_vars = create_memo(move |_| {
        let mut css_vars = String::new();
        theme.with(|theme| {
            css_vars.push_str(&format!(
                "--thaw-background-color: {};",
                theme.common.color_success
            ));
            css_vars.push_str(&format!(
                "--thaw-background-color-error: {};",
                theme.common.color_error
            ));
        });
        css_vars
    });
    let loading_bar_ref = create_node_ref::<html::Div>();
    let loading = create_rw_signal(false);

    let start = Callback::new(move |_| {
        loading.set(true);
        if let Some(loading_bar_ref) = loading_bar_ref.get_untracked() {
            let loading_bar_ref = loading_bar_ref
                .style("background-color", "var(--thaw-background-color)")
                .style("transition", "none")
                .style("max-width", "0");
            _ = loading_bar_ref.offset_width();
            loading_bar_ref
                .style("transition", "max-width 4s linear")
                .style("max-width", "80%");
        }
    });
    let is_on_transitionend = store_value(false);
    let on_transitionend = move |_| {
        if is_on_transitionend.get_value() {
            is_on_transitionend.set_value(false);
            loading.set(false);
        }
    };
    let finish = Callback::new(move |_| {
        if let Some(loading_bar_ref) = loading_bar_ref.get_untracked() {
            loading_bar_ref
                .style("background-color", "var(--thaw-background-color)")
                .style("transition", "max-width 0.5s linear")
                .style("max-width", "100%");
            is_on_transitionend.set_value(true);
        }
    });
    let error = Callback::new(move |_| {
        if let Some(loading_bar_ref) = loading_bar_ref.get_untracked() {
            if !loading.get() {
                loading.set(true);
                let loading_bar_ref = loading_bar_ref.clone();
                let loading_bar_ref = loading_bar_ref
                    .style("transition", "none")
                    .style("max-width", "0");
                _ = loading_bar_ref.offset_width();
            }
            loading_bar_ref
                .style("background-color", "var(--thaw-background-color-error)")
                .style("transition", "max-width 0.5s linear")
                .style("max-width", "100%");
            is_on_transitionend.set_value(true);
        }
    });

    comp_ref.load(LoadingBarRef {
        start,
        finish,
        error,
    });
    view! {
        <div
            class="thaw-loading-bar-container"
            style=move || (!loading.get()).then(|| "display: none;")
        >
            <div
                class="thaw-loading-bar"
                ref=loading_bar_ref
                style=move || css_vars.get()
                on:transitionend=on_transitionend
            ></div>
        </div>
    }
}
