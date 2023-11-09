use super::{LoadingBar, LoadingBarRef};
use crate::{teleport::Teleport, utils::ComponentRef};
use leptos::*;

#[component]
pub fn LoadingBarProvider(children: Children) -> impl IntoView {
    let loading_bar_ref = ComponentRef::<LoadingBarRef>::default();
    provide_context(LoadingBarInjection { loading_bar_ref });

    view! {
        {children()}
        <Teleport>
            <LoadingBar comp_ref=loading_bar_ref/>
        </Teleport>
    }
}

#[derive(Clone)]
pub struct LoadingBarInjection {
    loading_bar_ref: ComponentRef<LoadingBarRef>,
}

impl Copy for LoadingBarInjection {}

impl LoadingBarInjection {
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

pub fn use_loading_bar() -> LoadingBarInjection {
    expect_context::<LoadingBarInjection>()
}
