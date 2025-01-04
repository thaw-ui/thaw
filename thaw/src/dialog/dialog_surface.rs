use super::dialog::DialogInjection;
use leptos::prelude::*;
use thaw_components::CSSTransition;
use thaw_utils::class_list;

#[component]
pub fn DialogSurface(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let dialog = DialogInjection::expect_context();

    view! {
        <CSSTransition
            appear=dialog.open.get_untracked()
            show=dialog.open.signal()
            name="fade-in-scale-up-transition"
        >
            <div class=class_list!["thaw-dialog-surface", class] role="dialog" aria-modal="true">
                {children()}
            </div>
        </CSSTransition>
    }
}
