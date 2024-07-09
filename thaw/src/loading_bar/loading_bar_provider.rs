use super::{LoadingBar, LoadingBarRef};
use leptos::{context::Provider, prelude::*};
use thaw_components::Teleport;
use thaw_utils::ComponentRef;

#[component]
pub fn LoadingBarProvider(children: Children) -> impl IntoView {
    let loading_bar_ref = ComponentRef::<LoadingBarRef>::default();

    view! {
        <Provider value=LoadingBarInjection {
            loading_bar_ref,
        }>
            {children()} <Teleport>
                <LoadingBar comp_ref=loading_bar_ref/>
            </Teleport>
        </Provider>
    }
}

#[derive(Clone)]
pub struct LoadingBarInjection {
    loading_bar_ref: ComponentRef<LoadingBarRef>,
}

impl Copy for LoadingBarInjection {}

impl LoadingBarInjection {
    pub fn expect_use() -> Self {
        expect_context::<Self>()
    }

    pub fn start(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.start();
        }
    }

    pub fn finish(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.finish();
        }
    }

    pub fn error(&self) {
        if let Some(loading_bar_ref) = self.loading_bar_ref.get_untracked() {
            loading_bar_ref.error();
        }
    }
}
