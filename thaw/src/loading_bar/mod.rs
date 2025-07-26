mod loading_bar_provider;

pub use loading_bar_provider::*;

use crate::ConfigInjection;
use leptos::{html, prelude::*};
use std::sync::Arc;
use thaw_utils::ComponentRef;

#[cfg(feature = "manganis")]
const _: manganis::Asset = manganis::asset!("/src/loading_bar/loading-bar.css");

#[derive(Clone)]
pub(crate) struct LoadingBarRef {
    start: Arc<dyn Fn() + Send + Sync + 'static>,
    finish: Arc<dyn Fn() + Send + Sync + 'static>,
    error: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl LoadingBarRef {
    #[inline]
    pub fn start(&self) {
        (self.start)();
    }
    #[inline]
    pub fn finish(&self) {
        (self.finish)();
    }
    #[inline]
    pub fn error(&self) {
        (self.error)();
    }
}

#[component]
fn LoadingBar(#[prop(optional)] comp_ref: ComponentRef<LoadingBarRef>) -> impl IntoView {
    #[cfg(not(feature = "manganis"))]
    thaw_utils::mount_style("loading-bar", include_str!("./loading-bar.css"));

    let config_provider = ConfigInjection::expect_context();
    let container_ref = NodeRef::<html::Div>::new();
    let loading_bar_ref = NodeRef::<html::Div>::new();

    let start = Arc::new(move || {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        let _ = container_el.set_attribute("style", "");
        loading_bar_el.style(("background-color", "var(--colorStatusSuccessForeground1)"));
        loading_bar_el.style(("transition", "none"));
        loading_bar_el.style(("max-width", "0"));
        let _ = loading_bar_el.offset_width();
        loading_bar_el.style(("transition", "max-width 4s linear"));
        loading_bar_el.style(("max-width", "80%"));
    });
    let on_transitionend = move |_| {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        if let Ok(max_width) = Dom::style(&loading_bar_el).get_property_value("max-width") {
            if max_width == "100%" {
                let _ = container_el.set_attribute("style", "display: none");
            }
        }
    };
    let finish = Arc::new(move || {
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        loading_bar_el.style(("background-color", "var(--colorStatusSuccessForeground1)"));
        loading_bar_el.style(("transition", "max-width 0.5s linear"));
        loading_bar_el.style(("max-width", "100%"));
    });
    let error = Arc::new(move || {
        let Some(container_el) = container_ref.get_untracked() else {
            return;
        };
        let Some(loading_bar_el) = loading_bar_ref.get_untracked() else {
            return;
        };
        if container_el.get_attribute("style") != Some(String::new()) {
            let _ = container_el.set_attribute("style", "");
            loading_bar_el.style(("transition", "none"));
            loading_bar_el.style(("max-width", "0"));
            let _ = loading_bar_el.offset_width();
        }
        loading_bar_el.style(("background-color", "var(--colorStatusDangerForeground1)"));
        loading_bar_el.style(("transition", "max-width 0.5s linear"));
        loading_bar_el.style(("max-width", "100%"));
    });

    comp_ref.load(LoadingBarRef {
        start,
        finish,
        error,
    });
    view! {
        <div
            class="thaw-config-provider thaw-loading-bar-container"
            data-thaw-id=config_provider.id()
            style="display: none"
            node_ref=container_ref
        >
            <div
                class="thaw-loading-bar"
                node_ref=loading_bar_ref
                on:transitionend=on_transitionend
            ></div>
        </div>
    }
}
