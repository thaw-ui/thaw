mod loading_bar_provider;

pub use loading_bar_provider::{use_loading_bar, LoadingBarProvider};

use crate::ConfigInjection;
use leptos::*;
use thaw_utils::{mount_style, ComponentRef};

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
    let config_provider = ConfigInjection::use_();
    let loading_bar_ref = NodeRef::<html::Div>::new();
    let loading = RwSignal::new(false);

    let start = Callback::new(move |_| {
        loading.set(true);
        if let Some(loading_bar_ref) = loading_bar_ref.get_untracked() {
            let loading_bar_ref = loading_bar_ref
                .style("background-color", "var(--colorStatusSuccessForeground1)")
                .style("transition", "none")
                .style("max-width", "0");
            _ = loading_bar_ref.offset_width();
            _ = loading_bar_ref
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
            _ = loading_bar_ref
                .style("background-color", "var(--colorStatusSuccessForeground1)")
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
            _ = loading_bar_ref
                .style("background-color", "var(--colorStatusDangerForeground1)")
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
            class="thaw-config-provider thaw-loading-bar-container"
            data-thaw-id=config_provider.id().clone()
            style=move || (!loading.get()).then_some("display: none;")
        >
            <div
                class="thaw-loading-bar"
                ref=loading_bar_ref
                on:transitionend=on_transitionend
            ></div>
        </div>
    }
}
