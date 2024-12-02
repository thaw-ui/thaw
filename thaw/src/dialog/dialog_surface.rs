use super::dialog::DialogInjection;
use leptos::prelude::*;
use thaw_components::CSSTransition;
use thaw_utils::class_list;

#[component]
pub fn DialogSurface(
    #[prop(optional, into)] class: MaybeProp<String>,
    children: Children,
) -> impl IntoView {
    let dialog = DialogInjection::expect_use();

    view! {
        <CSSTransition
            appear=dialog.open.get_untracked()
            show=dialog.open.signal()
            name="fade-in-scale-up-transition"
            let:display
        >
            <div
                class=class_list!["thaw-dialog-surface", class]
                role="dialog"
                aria-modal="true"
                style:display=move || display.get().map(|_| "none").unwrap_or_default()
            >
                {children()}
            </div>
        </CSSTransition>
    }
}
