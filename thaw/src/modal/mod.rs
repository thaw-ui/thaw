use crate::{Card, CardFooter, CardHeader, CardHeaderExtra, Icon};
use leptos::{leptos_dom::helpers::WindowListenerHandle, *};
use thaw_components::{CSSTransition, OptionComp, Teleport};
use thaw_utils::{mount_style, use_click_position, Model};

#[slot]
pub struct ModalFooter {
    children: ChildrenFn,
}

#[component]
pub fn Modal(
    #[prop(into)] show: Model<bool>,
    #[prop(default = true.into(), into)] mask_closeable: MaybeSignal<bool>,
    #[prop(default = true, into)] close_on_esc: bool,
    #[prop(default = 2000.into(), into)] z_index: MaybeSignal<i16>,
    #[prop(default = MaybeSignal::Static("600px".to_string()), into)] width: MaybeSignal<String>,
    #[prop(optional, into)] title: MaybeSignal<String>,
    children: Children,
    #[prop(optional)] modal_footer: Option<ModalFooter>,
) -> impl IntoView {
    mount_style("modal", include_str!("./modal.css"));

    let displayed = RwSignal::new(show.get_untracked());
    let esc_handle = StoredValue::new(None::<WindowListenerHandle>);
    Effect::new(move |prev| {
        let is_show = show.get();
        if prev.is_some() && is_show {
            displayed.set(true);
        }

        if close_on_esc {
            if is_show && !prev.unwrap_or(false) {
                let handle = window_event_listener(ev::keydown, move |e| {
                    if &e.code() == "Escape" {
                        show.set(false);
                    }
                });
                esc_handle.set_value(Some(handle));
            } else {
                esc_handle.update_value(|handle| {
                    if let Some(handle) = handle.take() {
                        handle.remove();
                    }
                })
            }
        }
        is_show
    });

    let on_mask_click = move |_| {
        if mask_closeable.get_untracked() {
            show.set(false);
        }
    };

    let mask_ref = NodeRef::<html::Div>::new();
    let scroll_ref = NodeRef::<html::Div>::new();
    let modal_ref = NodeRef::<html::Div>::new();

    let click_position = use_click_position();
    let on_enter = Callback::new(move |_| {
        let Some(position) = click_position.get_untracked() else {
            return;
        };

        let Some(scroll_el) = scroll_ref.get_untracked() else {
            return;
        };
        let scroll_top = scroll_el.scroll_top();

        let Some(modal_el) = modal_ref.get_untracked() else {
            return;
        };

        let x = -(modal_el.offset_left() - position.0);
        let y = -(modal_el.offset_top() - position.1 - scroll_top);

        let _ = modal_el.attr("style", format!("transform-origin: {}px {}px", x, y));
    });

    on_cleanup(move || {
        esc_handle.update_value(|handle| {
            if let Some(handle) = handle.take() {
                handle.remove();
            }
        })
    });

    view! {
        <Teleport>
            <div
                class="thaw-modal-container"
                style:z-index=move || z_index.get()
                style=("--thaw-width", move || width.get())
            >
                <CSSTransition
                    node_ref=mask_ref
                    show=show.signal()
                    name="fade-in-transition"
                    let:display
                >
                    <div
                        class="thaw-modal-mask"
                        style=move || display.get()
                        on:click=on_mask_click
                        ref=mask_ref
                    ></div>
                </CSSTransition>
                <div
                    class="thaw-modal-scroll"
                    style=move || (!displayed.get()).then_some("display: none")
                    ref=scroll_ref
                >
                    <CSSTransition
                        node_ref=modal_ref
                        show=show.signal()
                        name="fade-in-scale-up-transition"
                        on_enter
                        on_after_leave=move |_| displayed.set(false)
                        let:display
                    >
                        <div
                            class="thaw-modal-body"
                            ref=modal_ref
                            role="dialog"
                            aria-modal="true"
                            style=move || display.get()
                        >
                            <Card>
                                <CardHeader slot>
                                    <span class="thaw-model-title">{move || title.get()}</span>
                                </CardHeader>
                                <CardHeaderExtra slot>
                                    <span
                                        style="cursor: pointer;"
                                        on:click=move |_| show.set(false)
                                    >
                                        <Icon icon=icondata_ai::AiCloseOutlined/>
                                    </span>
                                </CardHeaderExtra>
                                {children()}
                                <CardFooter slot if_=modal_footer.is_some()>
                                    <OptionComp value=modal_footer.as_ref() let:footer>
                                        {(footer.children)()}
                                    </OptionComp>
                                </CardFooter>
                            </Card>
                        </div>
                    </CSSTransition>
                </div>
            </div>
        </Teleport>
    }
}
