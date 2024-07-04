use crate::ConfigInjection;
use leptos::{context::Provider, ev, html, prelude::*};
use thaw_components::{CSSTransition, FocusTrap, Teleport};
use thaw_utils::{mount_style, Model};

#[component]
pub fn Dialog(
    #[prop(into)] open: Model<bool>,
    #[prop(default = true.into(), into)] mask_closeable: MaybeSignal<bool>,
    #[prop(default = true, into)] close_on_esc: bool,
    children: Children,
) -> impl IntoView {
    mount_style("dialog", include_str!("./dialog.css"));
    let config_provider = ConfigInjection::use_();

    let mask_ref = NodeRef::<html::Div>::new();
    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            open.set(false);
        }
    };
    let on_esc = Callback::new(move |_: ev::KeyboardEvent| {
        open.set(false);
    });

    view! {
        <Teleport immediate=open.signal()>
            <FocusTrap disabled=!close_on_esc active=open.signal() on_esc>
                <div
                    class="thaw-config-provider thaw-dialog"
                    data-thaw-id=config_provider.id().clone()
                >
                    <CSSTransition
                        node_ref=mask_ref
                        appear=open.get_untracked()
                        show=open.signal()
                        name="fade-in-transition"
                        let:display
                    >
                        <div
                            class="thaw-dialog-surface__backdrop"
                            style=move || display.get()
                            on:click=on_mask_click
                            node_ref=mask_ref
                            aria-hidden="true"
                        ></div>
                    </CSSTransition>
                    <Provider value=DialogInjection{open}>
                        {children()}
                    </Provider>
                </div>
            </FocusTrap>
        </Teleport>
    }
}

#[derive(Clone)]
pub(super) struct DialogInjection {
    pub open: Model<bool>,
}

impl DialogInjection {
    pub fn expect_use() -> Self {
        expect_context::<Self>()
    }
}
