use super::dialog::DialogInjection;
use leptos::{html, prelude::*};
use thaw_components::CSSTransition;

#[component]
pub fn DialogSurface(children: Children) -> impl IntoView {
    let dialog = DialogInjection::expect_use();
    let surface_ref = NodeRef::<html::Div>::new();

    view! {
        <CSSTransition
            node_ref=surface_ref
            appear=dialog.open.get_untracked()
            show=dialog.open.signal()
            name="fade-in-scale-up-transition"
            let:display
        >
            <div
                class="thaw-dialog-surface"
                node_ref=surface_ref
                role="dialog"
                aria-modal="true"
                style:display=move || display.get().map(|_| "none")
            >
                {children()}
            </div>
        </CSSTransition>
    }
}
